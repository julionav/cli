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
pub struct InlineObject12 {
	#[serde(rename = "display_name")]
	pub display_name: String,
	#[serde(rename = "name_id")]
	pub name_id: String,
}

impl InlineObject12 {
	pub fn new(display_name: String, name_id: String) -> InlineObject12 {
		InlineObject12 {
			display_name,
			name_id,
		}
	}
}
