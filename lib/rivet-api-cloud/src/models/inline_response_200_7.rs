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
pub struct InlineResponse2007 {
	#[serde(rename = "token")]
	pub token: String,
}

impl InlineResponse2007 {
	pub fn new(token: String) -> InlineResponse2007 {
		InlineResponse2007 { token }
	}
}
