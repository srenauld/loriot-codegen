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
pub struct LoriotUpDownLinkNoPayload {
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<f32>,
    #[serde(rename = "unconfirmed", skip_serializing_if = "Option::is_none")]
    pub unconfirmed: Option<f32>,
}

impl LoriotUpDownLinkNoPayload {
    pub fn new() -> LoriotUpDownLinkNoPayload {
        LoriotUpDownLinkNoPayload {
            confirmed: None,
            unconfirmed: None,
        }
    }
}


