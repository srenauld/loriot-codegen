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
pub struct LoriotInlineResponse2005 {
    /// number of frames downlink
    #[serde(rename = "dnfcn", skip_serializing_if = "Option::is_none")]
    pub dnfcn: Option<f32>,
    /// number of bytes downlink
    #[serde(rename = "dnlen", skip_serializing_if = "Option::is_none")]
    pub dnlen: Option<f32>,
    /// number of frames uplink
    #[serde(rename = "fcn", skip_serializing_if = "Option::is_none")]
    pub fcn: Option<f32>,
    /// number of bytes uplink
    #[serde(rename = "len", skip_serializing_if = "Option::is_none")]
    pub len: Option<f32>,
    /// number of lost frames
    #[serde(rename = "lst", skip_serializing_if = "Option::is_none")]
    pub lst: Option<f32>,
    /// record identifier in RFC
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "downlink", skip_serializing_if = "Option::is_none")]
    pub downlink: Option<crate::models::LoriotUpDownLink>,
    #[serde(rename = "uplink", skip_serializing_if = "Option::is_none")]
    pub uplink: Option<crate::models::LoriotUpDownLink>,
}

impl LoriotInlineResponse2005 {
    pub fn new() -> LoriotInlineResponse2005 {
        LoriotInlineResponse2005 {
            dnfcn: None,
            dnlen: None,
            fcn: None,
            len: None,
            lst: None,
            _id: None,
            downlink: None,
            uplink: None,
        }
    }
}


