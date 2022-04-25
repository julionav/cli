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
pub struct ValidationErrors {
	#[serde(rename = "errors")]
	pub errors: Vec<Vec<String>>,
}

impl ValidationErrors {
	pub fn new(errors: Vec<Vec<String>>) -> ValidationErrors {
		ValidationErrors { errors }
	}
}
