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
pub struct LoriotOutputDetailForm {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl LoriotOutputDetailForm {
    pub fn new() -> LoriotOutputDetailForm {
        LoriotOutputDetailForm {
            name: None,
            title: None,
            _type: None,
            value: None,
        }
    }
}


