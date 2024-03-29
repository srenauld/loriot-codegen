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
pub struct LoriotInlineObject7 {
    /// Device title, EUI by default
    #[serde(rename = "title")]
    pub title: String,
    /// Device description, empty by default
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// DevEUI is a global end-device ID in IEEE EUI64 address space that uniquely identifies the end-device. 
    #[serde(rename = "deveui")]
    pub deveui: String,
    /// NwkSKey is a network session key specific for the end-device. It is used by both the network server and the end-device to calculate and verify the MIC (message integrity code) of all data messages to ensure data integrity. 
    #[serde(rename = "nwkskey")]
    pub nwkskey: String,
    /// DevAddr consists of 32 bits identifies the end-device within the current network. 
    #[serde(rename = "devaddr")]
    pub devaddr: String,
    /// Uplink sequence number to initialize the device. 
    #[serde(rename = "seqno")]
    pub seqno: f32,
    /// Downlink sequence number to initialize the device. 
    #[serde(rename = "seqdn")]
    pub seqdn: f32,
}

impl LoriotInlineObject7 {
    pub fn new(title: String, deveui: String, nwkskey: String, devaddr: String, seqno: f32, seqdn: f32) -> LoriotInlineObject7 {
        LoriotInlineObject7 {
            title,
            description: None,
            deveui,
            nwkskey,
            devaddr,
            seqno,
            seqdn,
        }
    }
}


