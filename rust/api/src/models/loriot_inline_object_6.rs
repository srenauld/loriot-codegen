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
pub struct LoriotInlineObject6 {
    /// global ID in IEEE EUI64 address space that uniquely identifies the end-device.
    #[serde(rename = "deveui", skip_serializing_if = "Option::is_none")]
    pub deveui: Option<String>,
}

impl LoriotInlineObject6 {
    pub fn new() -> LoriotInlineObject6 {
        LoriotInlineObject6 {
            deveui: None,
        }
    }
}


