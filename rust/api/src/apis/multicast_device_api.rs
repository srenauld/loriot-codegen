/*
 * Public NWK REST API
 *
 * Public LORIOT Network Server NWK REST API documentation
 *
 * The version of the OpenAPI document: 5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::sync::Arc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use futures::{Future};
use reqwest;

use super::{Error, configuration};

pub struct MulticastDeviceApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl MulticastDeviceApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> MulticastDeviceApiClient {
        MulticastDeviceApiClient {
            configuration,
        }
    }
}

pub trait MulticastDeviceApi {
    fn loriot_1_nwk_app_appid_mcast_device_get(&self, APPID: &str, page: f32, per_page: f32, filter: Option<&str>, sort: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::LoriotMcastdev>, Error = Error> + Send>;
    fn loriot_1_nwk_app_appid_mcast_device_mcast_eui_delete(&self, APPID: &str, mcast_eui: &str) -> Box<dyn Future<Item = (), Error = Error> + Send>;
    fn loriot_1_nwk_app_appid_mcast_device_mcast_eui_put(&self, APPID: &str, mcast_eui: &str, loriot_mcast: Option<crate::models::LoriotMcast>) -> Box<dyn Future<Item = (), Error = Error> + Send>;
    fn loriot_1_nwk_app_appid_mcast_device_post(&self, APPID: &str, loriot_mcast: Option<crate::models::LoriotMcast>) -> Box<dyn Future<Item = crate::models::LoriotMcastdev, Error = Error> + Send>;
    fn loriot_1_nwk_app_appid_mcastdevlimit_post(&self, APPID: &str, loriot_inline_object2: Option<crate::models::LoriotInlineObject2>) -> Box<dyn Future<Item = crate::models::LoriotInlineResponse2001, Error = Error> + Send>;
}

impl MulticastDeviceApi for MulticastDeviceApiClient {
    fn loriot_1_nwk_app_appid_mcast_device_get(&self, APPID: &str, page: f32, per_page: f32, filter: Option<&str>, sort: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::LoriotMcastdev>, Error = Error> + Send> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/1/nwk/app/{APPID}/mcast-device", configuration.base_path, APPID=crate::apis::urlencode(APPID));
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("page", &page.to_string())]);
        req_builder = req_builder.query(&[("perPage", &per_page.to_string())]);
        if let Some(ref s) = filter {
            req_builder = req_builder.query(&[("filter", &s.to_string())]);
        }
        if let Some(ref s) = sort {
            req_builder = req_builder.query(&[("sort", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        Box::new(req_builder.send()
            .and_then(|i| i.error_for_status())
            .and_then(|mut i| i.json())
            .from_err()
        )
    }

    fn loriot_1_nwk_app_appid_mcast_device_mcast_eui_delete(&self, APPID: &str, mcast_eui: &str) -> Box<dyn Future<Item = (), Error = Error> + Send> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/1/nwk/app/{APPID}/mcast-device/{mcastEUI}", configuration.base_path, APPID=crate::apis::urlencode(APPID), mcastEUI=crate::apis::urlencode(mcast_eui));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        Box::new(req_builder.send()
            .and_then(|i| i.error_for_status())
            .map(|_| ())
            .from_err()
        )
    }

    fn loriot_1_nwk_app_appid_mcast_device_mcast_eui_put(&self, APPID: &str, mcast_eui: &str, loriot_mcast: Option<crate::models::LoriotMcast>) -> Box<dyn Future<Item = (), Error = Error> + Send> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/1/nwk/app/{APPID}/mcast-device/{mcastEUI}", configuration.base_path, APPID=crate::apis::urlencode(APPID), mcastEUI=crate::apis::urlencode(mcast_eui));
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };
        req_builder = req_builder.json(&loriot_mcast);

        // send request
        Box::new(req_builder.send()
            .and_then(|i| i.error_for_status())
            .map(|_| ())
            .from_err()
        )
    }

    fn loriot_1_nwk_app_appid_mcast_device_post(&self, APPID: &str, loriot_mcast: Option<crate::models::LoriotMcast>) -> Box<dyn Future<Item = crate::models::LoriotMcastdev, Error = Error> + Send> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/1/nwk/app/{APPID}/mcast-device", configuration.base_path, APPID=crate::apis::urlencode(APPID));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };
        req_builder = req_builder.json(&loriot_mcast);

        // send request
        Box::new(req_builder.send()
            .and_then(|i| i.error_for_status())
            .and_then(|mut i| i.json())
            .from_err()
        )
    }

    fn loriot_1_nwk_app_appid_mcastdevlimit_post(&self, APPID: &str, loriot_inline_object2: Option<crate::models::LoriotInlineObject2>) -> Box<dyn Future<Item = crate::models::LoriotInlineResponse2001, Error = Error> + Send> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/1/nwk/app/{APPID}/mcastdevlimit", configuration.base_path, APPID=crate::apis::urlencode(APPID));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };
        req_builder = req_builder.json(&loriot_inline_object2);

        // send request
        Box::new(req_builder.send()
            .and_then(|i| i.error_for_status())
            .and_then(|mut i| i.json())
            .from_err()
        )
    }

}
