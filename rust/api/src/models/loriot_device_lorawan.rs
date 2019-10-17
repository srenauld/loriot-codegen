/*
 * Public NWK REST API
 *
 * Public LORIOT Network Server NWK REST API documentation
 *
 * The version of the OpenAPI document: 5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LoriotDeviceLorawan : Device LoRaWAN standard used by device



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LoriotDeviceLorawan {
    /// Major version number of the LoRaWAN standard
    #[serde(rename = "major", skip_serializing_if = "Option::is_none")]
    pub major: Option<f32>,
    /// Minor version of the LoRaWAN standard
    #[serde(rename = "minor", skip_serializing_if = "Option::is_none")]
    pub minor: Option<f32>,
    /// Revision of the LoRaWAN standard (typically regional parameters)
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

impl LoriotDeviceLorawan {
    /// Device LoRaWAN standard used by device
    pub fn new() -> LoriotDeviceLorawan {
        LoriotDeviceLorawan {
            major: None,
            minor: None,
            revision: None,
        }
    }
}


