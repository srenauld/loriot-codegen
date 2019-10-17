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
pub struct LoriotInlineObject15 {
    /// Empty object
    #[serde(rename = "dutycycle", skip_serializing_if = "Option::is_none")]
    pub dutycycle: Option<serde_json::Value>,
}

impl LoriotInlineObject15 {
    pub fn new() -> LoriotInlineObject15 {
        LoriotInlineObject15 {
            dutycycle: None,
        }
    }
}


