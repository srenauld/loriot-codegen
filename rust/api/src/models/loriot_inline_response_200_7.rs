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
pub struct LoriotInlineResponse2007 {
    /// Last value counters by date
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "gws", skip_serializing_if = "Option::is_none")]
    pub gws: Option<Vec<crate::models::Loriot1NwkAppAppidDeviceDeveuiRadioFreqGws>>,
}

impl LoriotInlineResponse2007 {
    pub fn new() -> LoriotInlineResponse2007 {
        LoriotInlineResponse2007 {
            date: None,
            gws: None,
        }
    }
}


