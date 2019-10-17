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
pub struct LoriotGateway {
    /// Gateway EUI ID
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    /// Gateway MAC
    #[serde(rename = "MAC", skip_serializing_if = "Option::is_none")]
    pub MAC: Option<String>,
    /// Gateway EUI
    #[serde(rename = "EUI", skip_serializing_if = "Option::is_none")]
    pub EUI: Option<String>,
    /// base / manufacturer of the gateway
    #[serde(rename = "base", skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// model of the gateway
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<crate::models::LoriotLocation>,
    /// Gateway connection status
    #[serde(rename = "connected", skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    /// gateway channel ID
    #[serde(rename = "radioband", skip_serializing_if = "Option::is_none")]
    pub radioband: Option<String>,
    /// bus used to connect the concentrator
    #[serde(rename = "bus", skip_serializing_if = "Option::is_none")]
    pub bus: Option<String>,
    /// model of the concentrator
    #[serde(rename = "concentrator", skip_serializing_if = "Option::is_none")]
    pub concentrator: Option<String>,
    /// ID of the concentrator / bus / card
    #[serde(rename = "card", skip_serializing_if = "Option::is_none")]
    pub card: Option<f32>,
    /// date and time of last data
    #[serde(rename = "lastData", skip_serializing_if = "Option::is_none")]
    pub last_data: Option<String>,
    /// Date of last gateway start of the binary / full reload
    #[serde(rename = "lastStarted", skip_serializing_if = "Option::is_none")]
    pub last_started: Option<String>,
    /// last keep-alive timestamp
    #[serde(rename = "lastPong", skip_serializing_if = "Option::is_none")]
    pub last_pong: Option<String>,
    /// version reported by the gateway
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// latency (round trip time) between Network Server and Gateway from WebSocket protocol 
    #[serde(rename = "rtt", skip_serializing_if = "Option::is_none")]
    pub rtt: Option<f32>,
    /// custom gateway title updated by user
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// user friendly version of base
    #[serde(rename = "basename", skip_serializing_if = "Option::is_none")]
    pub basename: Option<String>,
    /// user friendly version of model
    #[serde(rename = "modelname", skip_serializing_if = "Option::is_none")]
    pub modelname: Option<String>,
    /// user friendly version of concentrator
    #[serde(rename = "concentratorname", skip_serializing_if = "Option::is_none")]
    pub concentratorname: Option<String>,
    /// flag enable / disable alerts for this gateway
    #[serde(rename = "alerts", skip_serializing_if = "Option::is_none")]
    pub alerts: Option<bool>,
    /// Creation date
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// visibility of the gateway. Organization wide or only for the user
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl LoriotGateway {
    pub fn new() -> LoriotGateway {
        LoriotGateway {
            _id: None,
            MAC: None,
            EUI: None,
            base: None,
            model: None,
            location: None,
            connected: None,
            radioband: None,
            bus: None,
            concentrator: None,
            card: None,
            last_data: None,
            last_started: None,
            last_pong: None,
            version: None,
            rtt: None,
            title: None,
            basename: None,
            modelname: None,
            concentratorname: None,
            alerts: None,
            created_at: None,
            visibility: None,
        }
    }
}


