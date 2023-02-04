/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudGamesExportMatchmakerLobbyHistoryInput {
    /// Unsigned 64 bit integer.
    #[serde(rename = "query_start", skip_serializing_if = "Option::is_none")]
    pub query_start: Option<f64>,
    /// Unsigned 64 bit integer.
    #[serde(rename = "query_end", skip_serializing_if = "Option::is_none")]
    pub query_end: Option<f64>,
}

impl CloudGamesExportMatchmakerLobbyHistoryInput {
    pub fn new() -> CloudGamesExportMatchmakerLobbyHistoryInput {
        CloudGamesExportMatchmakerLobbyHistoryInput {
            query_start: None,
            query_end: None,
        }
    }
}


