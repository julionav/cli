/*
 * Rivet Cloud
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineObject15 {
	#[serde(rename = "lobby_count_max")]
	pub lobby_count_max: i32,
	#[serde(rename = "max_players")]
	pub max_players: i32,
}

impl InlineObject15 {
	pub fn new(lobby_count_max: i32, max_players: i32) -> InlineObject15 {
		InlineObject15 {
			lobby_count_max,
			max_players,
		}
	}
}
