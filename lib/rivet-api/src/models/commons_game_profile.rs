/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommonsGameProfile : A game profile.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonsGameProfile {
    /// A universally unique identifier.
    #[serde(rename = "game_id")]
    pub game_id: String,
    /// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
    #[serde(rename = "name_id")]
    pub name_id: String,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// The URL of this game's logo image.
    #[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// The URL of this game's banner image.
    #[serde(rename = "banner_url", skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<String>,
    /// The URL to this game's website.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "developer")]
    pub developer: Box<crate::models::CommonsGroupSummary>,
    /// A list of game tags.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// A description of the given game.
    #[serde(rename = "description")]
    pub description: String,
    /// A list of platform links.
    #[serde(rename = "platforms")]
    pub platforms: Vec<crate::models::CommonsGamePlatformLink>,
    /// A list of group summaries.
    #[serde(rename = "recommended_groups")]
    pub recommended_groups: Vec<crate::models::CommonsGroupSummary>,
    /// A list of game leaderboard categories.
    #[serde(rename = "identity_leaderboard_categories")]
    pub identity_leaderboard_categories: Vec<crate::models::CommonsGameLeaderboardCategory>,
    /// A list of game leaderboard categories.
    #[serde(rename = "group_leaderboard_categories")]
    pub group_leaderboard_categories: Vec<crate::models::CommonsGameLeaderboardCategory>,
}

impl CommonsGameProfile {
    /// A game profile.
    pub fn new(game_id: String, name_id: String, display_name: String, url: String, developer: crate::models::CommonsGroupSummary, tags: Vec<String>, description: String, platforms: Vec<crate::models::CommonsGamePlatformLink>, recommended_groups: Vec<crate::models::CommonsGroupSummary>, identity_leaderboard_categories: Vec<crate::models::CommonsGameLeaderboardCategory>, group_leaderboard_categories: Vec<crate::models::CommonsGameLeaderboardCategory>) -> CommonsGameProfile {
        CommonsGameProfile {
            game_id,
            name_id,
            display_name,
            logo_url: None,
            banner_url: None,
            url,
            developer: Box::new(developer),
            tags,
            description,
            platforms,
            recommended_groups,
            identity_leaderboard_categories,
            group_leaderboard_categories,
        }
    }
}


