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
pub struct LoriotInlineObject10 {
    /// Receive window to be used after uplink transmission
    #[serde(rename = "rxw", skip_serializing_if = "Option::is_none")]
    pub rxw: Option<f32>,
    /// Flag to relax or strict uplink seqno check
    #[serde(rename = "seqrelax", skip_serializing_if = "Option::is_none")]
    pub seqrelax: Option<bool>,
    /// Device LoRaWAN class type
    #[serde(rename = "devclass", skip_serializing_if = "Option::is_none")]
    pub devclass: Option<String>,
    /// Minimum data rate to use for device when ADR is enabled
    #[serde(rename = "adrMin", skip_serializing_if = "Option::is_none")]
    pub adr_min: Option<f32>,
    /// Maximum data rate to use for device when ADR is enabled
    #[serde(rename = "adrMax", skip_serializing_if = "Option::is_none")]
    pub adr_max: Option<f32>,
    /// Fixed value for data rate when ADR is enabled
    #[serde(rename = "adrFix", skip_serializing_if = "Option::is_none")]
    pub adr_fix: Option<f32>,
    /// Device title, EUI by default
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Device description, empty by default
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Flag to reset sequence downlink when old uplink seqno received
    #[serde(rename = "seqdnreset", skip_serializing_if = "Option::is_none")]
    pub seqdnreset: Option<bool>,
    /// Enable/disable downlinks for device
    #[serde(rename = "canSend", skip_serializing_if = "Option::is_none")]
    pub can_send: Option<bool>,
    /// Enable/disable payload in a downlink for device
    #[serde(rename = "canSendPayload", skip_serializing_if = "Option::is_none")]
    pub can_send_payload: Option<bool>,
    /// Enable/disable MAC in a downlink for device
    #[serde(rename = "canSendFOPTS", skip_serializing_if = "Option::is_none")]
    pub can_send_fopts: Option<bool>,
}

impl LoriotInlineObject10 {
    pub fn new() -> LoriotInlineObject10 {
        LoriotInlineObject10 {
            rxw: None,
            seqrelax: None,
            devclass: None,
            adr_min: None,
            adr_max: None,
            adr_fix: None,
            title: None,
            description: None,
            seqdnreset: None,
            can_send: None,
            can_send_payload: None,
            can_send_fopts: None,
        }
    }
}


