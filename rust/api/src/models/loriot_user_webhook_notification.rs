/*
 * Public NWK REST API
 *
 * Public LORIOT Network Server NWK REST API documentation
 *
 * The version of the OpenAPI document: 5.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LoriotUserWebhookNotification {
    /// User webhook URL notification end-point
    #[serde(rename = "webhookNotification", skip_serializing_if = "Option::is_none")]
    pub webhook_notification: Option<String>,
}

impl LoriotUserWebhookNotification {
    pub fn new() -> LoriotUserWebhookNotification {
        LoriotUserWebhookNotification {
            webhook_notification: None,
        }
    }
}


