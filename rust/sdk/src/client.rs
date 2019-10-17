use loriot_api::apis::configuration::{ Configuration, ApiKey };
use loriot_api::apis::Error;
use reqwest::async::{Client as InnerClient};
use std::io;
use futures_locks::{RwLock as FutLock};
use futures::{Future, future};
use std::sync::Arc;
use std::ops::Deref;

pub struct Session {
    token: (String, String)
}

pub enum APIError {
    AccessDenied,
    IncorrectCredentials,
    EndpointNotFound,
    InternalError,
    SynchronizationError,
    Unknown(String)
}
/*
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
} */

impl From<Error> for APIError {
    fn from(item: Error) -> APIError {
        APIError::InternalError
    }
}

pub trait SessionTokenMethod {
    fn authenticate(&self, server: String) -> Box<dyn Future<Item = (String, String), Error = APIError> + Send>;
}

#[derive(Clone)]
pub struct ClientConfiguration {
    pub server: String,
    pub authentication: Arc<Box<dyn SessionTokenMethod + Send + Sync>>
}

#[derive(Clone)]
pub struct Client {
    configuration: ClientConfiguration,
    session: Arc<FutLock<Option<Session>>>,
    client: InnerClient
}

impl Client {

    pub fn new(config: ClientConfiguration) -> Arc<Client> {
        Arc::new(Client {
            configuration: config,
            session: Arc::new(FutLock::new(None)),
            client: reqwest::async::Client::new()
        })
    }

    pub fn deauthenticate(&self) -> Box<dyn Future<Item = (), Error = APIError> + Send> {
        Box::new(self.session.write().map(|mut session| {
            session.take();
        })
        .map_err(|_| APIError::SynchronizationError)
        )
    }

    fn authenticate(&self) -> Box<dyn Future<Item = futures_locks::RwLockReadGuard<Option<Session>>, Error = APIError> + Send> {
        let read_copy = self.session.clone();
        let server = self.configuration.server.clone();
        let generator = self.configuration.authentication.clone();
        Box::new(self.session.write()
            .map_err(|_| APIError::SynchronizationError)
        .and_then(move |mut session| -> Box<dyn Future<Item = _, Error = _> + Send> {
            match session.deref().is_some() {
                true => Box::new(future::ok(())),
                false => Box::new(generator.authenticate(server).map(move |(token_type, token)| {
                    *session = Some(Session {
                        token: (token_type, token)
                    });
                }))
            }
        })
        .and_then(move |_| {
            read_copy.read()
                .map_err(|_| APIError::SynchronizationError)
        }))
    }

    pub fn configuration(&self) -> Box<dyn Future<Item = Configuration, Error = APIError> + Send> {
        let base_path = self.configuration.server.clone();
        let client = self.client.clone();
        Box::new(self.authenticate().map(|session| {
            Configuration {
                base_path: base_path,
                user_agent: Some("OpenAPI-Generator/5.0/rust".to_owned()),
                client: client,
                basic_auth: None,
                oauth_access_token: None,
                bearer_access_token: None,
                api_key: session.deref().as_ref().map(|session| ApiKey {
                    prefix: Some(session.token.0.clone()),
                    key: session.token.1.clone()
                }  )
            }
        }))
        
    }
}