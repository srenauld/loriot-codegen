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
pub struct LoriotPaginationDevices {
    /// Page number for the pagination response
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<f32>,
    /// Amount of applications to retrieve per page
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<f32>,
    /// Amount of applications to iterate using pagination
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f32>,
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::LoriotDevice>>,
}

impl LoriotPaginationDevices {
    pub fn new() -> LoriotPaginationDevices {
        LoriotPaginationDevices {
            page: None,
            per_page: None,
            total: None,
            devices: None,
        }
    }
}


