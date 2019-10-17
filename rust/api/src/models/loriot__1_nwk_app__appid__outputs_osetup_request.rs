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
pub struct Loriot1NwkAppAppidOutputsOsetupRequest {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl Loriot1NwkAppAppidOutputsOsetupRequest {
    pub fn new() -> Loriot1NwkAppAppidOutputsOsetupRequest {
        Loriot1NwkAppAppidOutputsOsetupRequest {
            token: None,
        }
    }
}


