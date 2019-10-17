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
pub struct LoriotInlineResponse2003 {
    /// application title
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl LoriotInlineResponse2003 {
    pub fn new() -> LoriotInlineResponse2003 {
        LoriotInlineResponse2003 {
            title: None,
        }
    }
}


