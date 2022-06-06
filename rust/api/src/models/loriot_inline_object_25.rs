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
pub struct LoriotInlineObject25 {
    #[serde(rename = "radioband", skip_serializing_if = "Option::is_none")]
    pub radioband: Option<String>,
}

impl LoriotInlineObject25 {
    pub fn new() -> LoriotInlineObject25 {
        LoriotInlineObject25 {
            radioband: None,
        }
    }
}

