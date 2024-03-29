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
pub struct LoriotUserEmailNotification {
    /// User email notification
    #[serde(rename = "emailNotification", skip_serializing_if = "Option::is_none")]
    pub email_notification: Option<String>,
}

impl LoriotUserEmailNotification {
    pub fn new() -> LoriotUserEmailNotification {
        LoriotUserEmailNotification {
            email_notification: None,
        }
    }
}


