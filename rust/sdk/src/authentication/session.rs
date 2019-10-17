use crate::client::SessionTokenMethod;
use reqwest::async::Client;
use std::collections::HashMap;
use futures::Future;
use std::io;
use crate::client::APIError;

pub struct SessionAuth(pub String, pub String);

#[derive(Deserialize)]
pub struct ParsedSession {
    session: String
}
impl SessionAuth {
    fn inner_auth(&self, server: String) -> Box<dyn Future<Item = ParsedSession, Error = APIError> + Send> {
        let client = Client::new();
        let body:HashMap<String, String> = vec![
            ("user".to_string(), self.0.clone()),
            ("pwd".to_string(), self.1.clone())
        ].into_iter().collect();
        Box::new(client.post(&format!("{}/1/pub/login", server))
            .json(&body)
                .send()
            .and_then(|i| i.error_for_status())
            .and_then(|mut i| i.json())
            .map_err(|e| {
                APIError::AccessDenied
            })
        )
    }
}
impl SessionTokenMethod for SessionAuth {
    fn authenticate(&self, server: String) -> Box<dyn Future<Item = (String, String), Error = APIError> + Send> {
        Box::new(self.inner_auth(server).map(|i| {
            ("Session".to_string(), i.session)
        }))
    }
}