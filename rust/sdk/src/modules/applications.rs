use crate::resource::{Paginated, Retrieve};
use crate::client::Client;
use std::sync::Arc;
use loriot_api::apis::ApplicationApiClient;
use loriot_api::models::LoriotApplication;
use resource::{Entity, Pagination};
use futures::{Stream, Future, stream};
use crate::client::APIError;
use loriot_api::apis::ApplicationApi;

pub struct Applications {
    client: Client
}

impl Applications {

    pub fn new(client: Client) -> Applications {
        Applications {
            client: client
        }
    }
}
impl Entity for LoriotApplication {
}
impl Paginated<LoriotApplication> for Applications {
    fn list(&self, settings: Pagination) -> Box<dyn Stream<Item = LoriotApplication, Error = APIError> + Send> {
        Box::new(self.client.configuration().and_then(move |config| {
            let (per_page, page) = (settings.items as f32, settings.page as f32);
            ApplicationApiClient::new(Arc::new(config))
                .loriot_1_nwk_apps_get(page, per_page, None, None)
                .from_err()
                .map(|r| r.apps)
        })
        .map(|e| stream::iter_ok(e))
        .into_stream()
        .flatten())
    }
}