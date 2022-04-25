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
pub struct VersionSummary {
	#[serde(rename = "version_id")]
	pub version_id: String,
	#[serde(rename = "create_ts")]
	pub create_ts: i64,
	#[serde(rename = "display_name")]
	pub display_name: String,
}

impl VersionSummary {
	pub fn new(version_id: String, create_ts: i64, display_name: String) -> VersionSummary {
		VersionSummary {
			version_id,
			create_ts,
			display_name,
		}
	}
}
