/*
 * Public NWK REST API
 *
 * Public LORIOT Network Server NWK REST API documentation
 *
 * The version of the OpenAPI document: 5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LoriotUser : User information



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LoriotUser {
    /// notifcations alerts configuration
    #[serde(rename = "alerts", skip_serializing_if = "Option::is_none")]
    pub alerts: Option<bool>,
    /// devices limit for this user
    #[serde(rename = "devlimit", skip_serializing_if = "Option::is_none")]
    pub devlimit: Option<f32>,
    /// email address
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// first name or forename
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// gateways limit for this user
    #[serde(rename = "gwlimit", skip_serializing_if = "Option::is_none")]
    pub gwlimit: Option<f32>,
    /// the user has a credit card in his account
    #[serde(rename = "hascard", skip_serializing_if = "Option::is_none")]
    pub hascard: Option<bool>,
    /// last name or surname
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// level of the user for admin rights (1 to 100)
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<f32>,
    /// multicast devices limit by user
    #[serde(rename = "mcastdevlimit", skip_serializing_if = "Option::is_none")]
    pub mcastdevlimit: Option<f32>,
    /// Organization role of the user
    #[serde(rename = "organizationRole", skip_serializing_if = "Option::is_none")]
    pub organization_role: Option<String>,
    /// Organization unique identifier
    #[serde(rename = "organizationUuid", skip_serializing_if = "Option::is_none")]
    pub organization_uuid: Option<String>,
    /// max number ouputs allowed
    #[serde(rename = "outputLimit", skip_serializing_if = "Option::is_none")]
    pub output_limit: Option<f32>,
    /// Tier of the user
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<f32>,
}

impl LoriotUser {
    /// User information
    pub fn new() -> LoriotUser {
        LoriotUser {
            alerts: None,
            devlimit: None,
            email: None,
            first_name: None,
            gwlimit: None,
            hascard: None,
            last_name: None,
            level: None,
            mcastdevlimit: None,
            organization_role: None,
            organization_uuid: None,
            output_limit: None,
            tier: None,
        }
    }
}


