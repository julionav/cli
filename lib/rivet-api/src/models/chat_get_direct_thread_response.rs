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
pub struct ChatGetDirectThreadResponse {
	#[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
	pub identity: Option<Box<crate::models::IdentityHandle>>,
	#[serde(rename = "thread_id", skip_serializing_if = "Option::is_none")]
	pub thread_id: Option<uuid::Uuid>,
}

impl ChatGetDirectThreadResponse {
	pub fn new() -> ChatGetDirectThreadResponse {
		ChatGetDirectThreadResponse {
			identity: None,
			thread_id: None,
		}
	}
}
