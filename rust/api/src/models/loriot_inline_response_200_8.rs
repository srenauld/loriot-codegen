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
pub struct LoriotInlineResponse2008 {
    /// number of frames downlink
    #[serde(rename = "dnfcn", skip_serializing_if = "Option::is_none")]
    pub dnfcn: Option<f32>,
    /// number of frames uplink
    #[serde(rename = "fcn", skip_serializing_if = "Option::is_none")]
    pub fcn: Option<f32>,
    /// number of lost frames
    #[serde(rename = "lst", skip_serializing_if = "Option::is_none")]
    pub lst: Option<f32>,
    /// Record identifier
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
}

impl LoriotInlineResponse2008 {
    pub fn new() -> LoriotInlineResponse2008 {
        LoriotInlineResponse2008 {
            dnfcn: None,
            fcn: None,
            lst: None,
            _id: None,
        }
    }
}

