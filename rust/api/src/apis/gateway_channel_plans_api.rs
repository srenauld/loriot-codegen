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

pub struct GatewayChannelPlansApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl GatewayChannelPlansApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> GatewayChannelPlansApiClient {
        GatewayChannelPlansApiClient {
            configuration,
        }
    }
}

pub trait GatewayChannelPlansApi {
    fn loriot_1_nwk_channelplans_loragwversion_regions_get(&self, LORAGWVERSION: f32) -> Box<dyn Future<Item = Vec<crate::models::LoriotInlineResponse20025>, Error = Error> + Send>;
    fn loriot_1_nwk_channelplans_loragwversionbandname_channels_get(&self, LORAGWVERSION: f32, BANDNAME: &str) -> Box<dyn Future<Item = Vec<crate::models::LoriotInlineResponse20026>, Error = Error> + Send>;
    fn loriot_1_nwk_channelplans_loragwversionbandname_rx2_get(&self, LORAGWVERSION: f32, BANDNAME: &str) -> Box<dyn Future<Item = crate::models::LoriotInlineResponse20027, Error = Error> + Send>;
}

impl GatewayChannelPlansApi for GatewayChannelPlansApiClient {
    fn loriot_1_nwk_channelplans_loragwversion_regions_get(&self, LORAGWVERSION: f32) -> Box<dyn Future<Item = Vec<crate::models::LoriotInlineResponse20025>, Error = Error> + Send> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/1/nwk/channelplans/{LORAGWVERSION}/regions", configuration.base_path, LORAGWVERSION=LORAGWVERSION);
        let mut req_builder = client.get(uri_str.as_str());

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

    fn loriot_1_nwk_channelplans_loragwversionbandname_channels_get(&self, LORAGWVERSION: f32, BANDNAME: &str) -> Box<dyn Future<Item = Vec<crate::models::LoriotInlineResponse20026>, Error = Error> + Send> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/1/nwk/channelplans/{LORAGWVERSION}/{BANDNAME}/channels", configuration.base_path, LORAGWVERSION=LORAGWVERSION, BANDNAME=crate::apis::urlencode(BANDNAME));
        let mut req_builder = client.get(uri_str.as_str());

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

    fn loriot_1_nwk_channelplans_loragwversionbandname_rx2_get(&self, LORAGWVERSION: f32, BANDNAME: &str) -> Box<dyn Future<Item = crate::models::LoriotInlineResponse20027, Error = Error> + Send> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/1/nwk/channelplans/{LORAGWVERSION}/{BANDNAME}/rx2", configuration.base_path, LORAGWVERSION=LORAGWVERSION, BANDNAME=crate::apis::urlencode(BANDNAME));
        let mut req_builder = client.get(uri_str.as_str());

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

}