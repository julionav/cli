/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommonsMatchmakerLobbyJoinInfoPlayer : A matchmaker lobby player.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonsMatchmakerLobbyJoinInfoPlayer {
    /// Documentation at https://jwt.io/
    #[serde(rename = "token")]
    pub token: String,
}

impl CommonsMatchmakerLobbyJoinInfoPlayer {
    /// A matchmaker lobby player.
    pub fn new(token: String) -> CommonsMatchmakerLobbyJoinInfoPlayer {
        CommonsMatchmakerLobbyJoinInfoPlayer {
            token,
        }
    }
}


