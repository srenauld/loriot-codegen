use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod application_api;
pub use self::application_api::{ ApplicationApi, ApplicationApiClient };
mod application_output_api;
pub use self::application_output_api::{ ApplicationOutputApi, ApplicationOutputApiClient };
mod devices_api;
pub use self::devices_api::{ DevicesApi, DevicesApiClient };
mod gateway_api;
pub use self::gateway_api::{ GatewayApi, GatewayApiClient };
mod gateway_channel_plans_api;
pub use self::gateway_channel_plans_api::{ GatewayChannelPlansApi, GatewayChannelPlansApiClient };
mod multicast_device_api;
pub use self::multicast_device_api::{ MulticastDeviceApi, MulticastDeviceApiClient };
mod network_api;
pub use self::network_api::{ NetworkApi, NetworkApiClient };
mod system_api;
pub use self::system_api::{ SystemApi, SystemApiClient };
mod user_api;
pub use self::user_api::{ UserApi, UserApiClient };

pub mod configuration;
pub mod client;
