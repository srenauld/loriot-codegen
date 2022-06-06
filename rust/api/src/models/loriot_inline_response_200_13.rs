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
pub struct LoriotInlineResponse20013 {
    /// Gateway EUI ID
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    /// Gateway MAC
    #[serde(rename = "MAC", skip_serializing_if = "Option::is_none")]
    pub MAC: Option<String>,
    /// Gateway EUI
    #[serde(rename = "EUI", skip_serializing_if = "Option::is_none")]
    pub EUI: Option<String>,
    /// base / manufacturer of the gateway
    #[serde(rename = "base", skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// model of the gateway
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl LoriotInlineResponse20013 {
    pub fn new() -> LoriotInlineResponse20013 {
        LoriotInlineResponse20013 {
            _id: None,
            MAC: None,
            EUI: None,
            base: None,
            model: None,
        }
    }
}

