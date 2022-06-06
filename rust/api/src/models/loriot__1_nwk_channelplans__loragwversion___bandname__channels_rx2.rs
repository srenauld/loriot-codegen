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
pub struct Loriot1NwkChannelplansLoragwversionBandnameChannelsRx2 {
    /// RX2 frequency
    #[serde(rename = "freq", skip_serializing_if = "Option::is_none")]
    pub freq: Option<f32>,
    /// RX2 delay
    #[serde(rename = "delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<f32>,
    /// RX2 datarate
    #[serde(rename = "datr", skip_serializing_if = "Option::is_none")]
    pub datr: Option<String>,
}

impl Loriot1NwkChannelplansLoragwversionBandnameChannelsRx2 {
    pub fn new() -> Loriot1NwkChannelplansLoragwversionBandnameChannelsRx2 {
        Loriot1NwkChannelplansLoragwversionBandnameChannelsRx2 {
            freq: None,
            delay: None,
            datr: None,
        }
    }
}

