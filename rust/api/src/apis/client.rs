use std::sync::Arc;

use super::configuration::Configuration;

pub struct APIClient {
    application_api: Box<dyn crate::apis::ApplicationApi>,
    application_output_api: Box<dyn crate::apis::ApplicationOutputApi>,
    devices_api: Box<dyn crate::apis::DevicesApi>,
    gateway_api: Box<dyn crate::apis::GatewayApi>,
    gateway_channel_plans_api: Box<dyn crate::apis::GatewayChannelPlansApi>,
    multicast_device_api: Box<dyn crate::apis::MulticastDeviceApi>,
    network_api: Box<dyn crate::apis::NetworkApi>,
    system_api: Box<dyn crate::apis::SystemApi>,
    user_api: Box<dyn crate::apis::UserApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Arc::new(configuration);

        APIClient {
            application_api: Box::new(crate::apis::ApplicationApiClient::new(rc.clone())),
            application_output_api: Box::new(crate::apis::ApplicationOutputApiClient::new(rc.clone())),
            devices_api: Box::new(crate::apis::DevicesApiClient::new(rc.clone())),
            gateway_api: Box::new(crate::apis::GatewayApiClient::new(rc.clone())),
            gateway_channel_plans_api: Box::new(crate::apis::GatewayChannelPlansApiClient::new(rc.clone())),
            multicast_device_api: Box::new(crate::apis::MulticastDeviceApiClient::new(rc.clone())),
            network_api: Box::new(crate::apis::NetworkApiClient::new(rc.clone())),
            system_api: Box::new(crate::apis::SystemApiClient::new(rc.clone())),
            user_api: Box::new(crate::apis::UserApiClient::new(rc.clone())),
        }
    }

    pub fn application_api(&self) -> &dyn crate::apis::ApplicationApi{
        self.application_api.as_ref()
    }

    pub fn application_output_api(&self) -> &dyn crate::apis::ApplicationOutputApi{
        self.application_output_api.as_ref()
    }

    pub fn devices_api(&self) -> &dyn crate::apis::DevicesApi{
        self.devices_api.as_ref()
    }

    pub fn gateway_api(&self) -> &dyn crate::apis::GatewayApi{
        self.gateway_api.as_ref()
    }

    pub fn gateway_channel_plans_api(&self) -> &dyn crate::apis::GatewayChannelPlansApi{
        self.gateway_channel_plans_api.as_ref()
    }

    pub fn multicast_device_api(&self) -> &dyn crate::apis::MulticastDeviceApi{
        self.multicast_device_api.as_ref()
    }

    pub fn network_api(&self) -> &dyn crate::apis::NetworkApi{
        self.network_api.as_ref()
    }

    pub fn system_api(&self) -> &dyn crate::apis::SystemApi{
        self.system_api.as_ref()
    }

    pub fn user_api(&self) -> &dyn crate::apis::UserApi{
        self.user_api.as_ref()
    }

}
