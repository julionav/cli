/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommonsAuthAgentGameCloud : The current authenticated game cloud.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonsAuthAgentGameCloud {
    #[serde(rename = "game_id")]
    pub game_id: uuid::Uuid,
}

impl CommonsAuthAgentGameCloud {
    /// The current authenticated game cloud.
    pub fn new(game_id: uuid::Uuid) -> CommonsAuthAgentGameCloud {
        CommonsAuthAgentGameCloud {
            game_id,
        }
    }
}


