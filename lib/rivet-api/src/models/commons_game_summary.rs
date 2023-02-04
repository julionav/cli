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
pub struct CommonsGameSummary {
    #[serde(rename = "game_id")]
    pub game_id: uuid::Uuid,
    /// A human readable short identifier used to references resources. Different than a `uuid` because this is intended to be human readable. Different than `DisplayName` because this should not include special characters and be short.
    #[serde(rename = "name_id")]
    pub name_id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// The URL of this game's logo image.
    #[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// The URL of this game's banner image.
    #[serde(rename = "banner_url", skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "developer")]
    pub developer: Box<crate::models::CommonsGroupHandle>,
}

impl CommonsGameSummary {
    pub fn new(game_id: uuid::Uuid, name_id: String, display_name: String, url: String, developer: crate::models::CommonsGroupHandle) -> CommonsGameSummary {
        CommonsGameSummary {
            game_id,
            name_id,
            display_name,
            logo_url: None,
            banner_url: None,
            url,
            developer: Box::new(developer),
        }
    }
}


