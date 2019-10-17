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
pub struct LoriotInlineResponse20019Uptime {
    /// computed round time trip (computed latency)
    #[serde(rename = "crtt", skip_serializing_if = "Option::is_none")]
    pub crtt: Option<f32>,
    /// date time
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// cumulated latency value
    #[serde(rename = "pong", skip_serializing_if = "Option::is_none")]
    pub pong: Option<f32>,
    /// round trip time
    #[serde(rename = "rtt", skip_serializing_if = "Option::is_none")]
    pub rtt: Option<f32>,
}

impl LoriotInlineResponse20019Uptime {
    pub fn new() -> LoriotInlineResponse20019Uptime {
        LoriotInlineResponse20019Uptime {
            crtt: None,
            date: None,
            pong: None,
            rtt: None,
        }
    }
}


