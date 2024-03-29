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
pub struct LoriotInlineResponse20017Traffic {
    /// Accumulated counters by data date
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Uplink frame counter
    #[serde(rename = "fcn", skip_serializing_if = "Option::is_none")]
    pub fcn: Option<f32>,
    /// Counter by datarate, \"sf\" : { \"7\": 2 } (sf = spreading factor) 
    #[serde(rename = "sf", skip_serializing_if = "Option::is_none")]
    pub sf: Option<serde_json::Value>,
    /// Accumulated counters by frequency, \"freq\": { \"867900000\": 1, \"868100000\": 1 } 
    #[serde(rename = "freq", skip_serializing_if = "Option::is_none")]
    pub freq: Option<serde_json::Value>,
}

impl LoriotInlineResponse20017Traffic {
    pub fn new() -> LoriotInlineResponse20017Traffic {
        LoriotInlineResponse20017Traffic {
            date: None,
            fcn: None,
            sf: None,
            freq: None,
        }
    }
}


