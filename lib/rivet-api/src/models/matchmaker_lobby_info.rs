/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MatchmakerLobbyInfo : A public lobby in the lobby list.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchmakerLobbyInfo {
    #[serde(rename = "region_id")]
    pub region_id: String,
    #[serde(rename = "game_mode_id")]
    pub game_mode_id: String,
    #[serde(rename = "lobby_id")]
    pub lobby_id: uuid::Uuid,
    #[serde(rename = "max_players_normal")]
    pub max_players_normal: i32,
    #[serde(rename = "max_players_direct")]
    pub max_players_direct: i32,
    #[serde(rename = "max_players_party")]
    pub max_players_party: i32,
    #[serde(rename = "total_player_count")]
    pub total_player_count: i32,
}

impl MatchmakerLobbyInfo {
    /// A public lobby in the lobby list.
    pub fn new(region_id: String, game_mode_id: String, lobby_id: uuid::Uuid, max_players_normal: i32, max_players_direct: i32, max_players_party: i32, total_player_count: i32) -> MatchmakerLobbyInfo {
        MatchmakerLobbyInfo {
            region_id,
            game_mode_id,
            lobby_id,
            max_players_normal,
            max_players_direct,
            max_players_party,
            total_player_count,
        }
    }
}


