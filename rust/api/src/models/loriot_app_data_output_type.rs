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
pub struct LoriotAppDataOutputType {
    /// Data Output ID
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    /// Form properties to be displayed in front-end
    #[serde(rename = "form", skip_serializing_if = "Option::is_none")]
    pub form: Option<Vec<serde_json::Value>>,
    /// Setup instructions, long texts
    #[serde(rename = "howto", skip_serializing_if = "Option::is_none")]
    pub howto: Option<String>,
    #[serde(rename = "mech", skip_serializing_if = "Option::is_none")]
    pub mech: Option<String>,
    /// Title
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Data Output origin (local from LORIOT or from 3rd party)
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl LoriotAppDataOutputType {
    pub fn new() -> LoriotAppDataOutputType {
        LoriotAppDataOutputType {
            _id: None,
            form: None,
            howto: None,
            mech: None,
            title: None,
            location: None,
        }
    }
}


