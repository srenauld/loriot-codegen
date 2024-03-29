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
pub struct LoriotInlineObject2 {
    /// multicast device limit
    #[serde(rename = "mcastdevlimit", skip_serializing_if = "Option::is_none")]
    pub mcastdevlimit: Option<f32>,
}

impl LoriotInlineObject2 {
    pub fn new() -> LoriotInlineObject2 {
        LoriotInlineObject2 {
            mcastdevlimit: None,
        }
    }
}


