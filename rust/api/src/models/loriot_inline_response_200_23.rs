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
pub struct LoriotInlineResponse20023 {
    /// record identifier
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    /// gateway identifier
    #[serde(rename = "gweui", skip_serializing_if = "Option::is_none")]
    pub gweui: Option<String>,
    /// date time
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "storage", skip_serializing_if = "Option::is_none")]
    pub storage: Option<Vec<crate::models::Loriot1NwkGatewayGweuiSysinfoStorage>>,
    /// ram free space
    #[serde(rename = "ramFree", skip_serializing_if = "Option::is_none")]
    pub ram_free: Option<f32>,
    /// ram total space size
    #[serde(rename = "ramSize", skip_serializing_if = "Option::is_none")]
    pub ram_size: Option<f32>,
    /// normal linux loads average 1 5 15
    #[serde(rename = "loads", skip_serializing_if = "Option::is_none")]
    pub loads: Option<Vec<f32>>,
    /// uptime in unix time
    #[serde(rename = "upTime", skip_serializing_if = "Option::is_none")]
    pub up_time: Option<f32>,
    /// number of CPUs
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<f32>,
}

impl LoriotInlineResponse20023 {
    pub fn new() -> LoriotInlineResponse20023 {
        LoriotInlineResponse20023 {
            _id: None,
            gweui: None,
            date: None,
            storage: None,
            ram_free: None,
            ram_size: None,
            loads: None,
            up_time: None,
            cpus: None,
        }
    }
}


