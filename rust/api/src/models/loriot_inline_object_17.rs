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
pub struct LoriotInlineObject17 {
    /// New user email notification
    #[serde(rename = "emailNotification")]
    pub email_notification: String,
}

impl LoriotInlineObject17 {
    pub fn new(email_notification: String) -> LoriotInlineObject17 {
        LoriotInlineObject17 {
            email_notification,
        }
    }
}

