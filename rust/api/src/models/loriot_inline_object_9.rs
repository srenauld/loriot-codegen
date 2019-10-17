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
pub struct LoriotInlineObject9 {
    #[serde(rename = "tagname", skip_serializing_if = "Option::is_none")]
    pub tagname: Option<String>,
}

impl LoriotInlineObject9 {
    pub fn new() -> LoriotInlineObject9 {
        LoriotInlineObject9 {
            tagname: None,
        }
    }
}


