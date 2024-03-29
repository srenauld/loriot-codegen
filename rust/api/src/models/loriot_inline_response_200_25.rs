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
pub struct LoriotInlineResponse20025 {
    /// Channel region identifier
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "gwchannels", skip_serializing_if = "Option::is_none")]
    pub gwchannels: Option<Vec<String>>,
}

impl LoriotInlineResponse20025 {
    pub fn new() -> LoriotInlineResponse20025 {
        LoriotInlineResponse20025 {
            _id: None,
            gwchannels: None,
        }
    }
}


