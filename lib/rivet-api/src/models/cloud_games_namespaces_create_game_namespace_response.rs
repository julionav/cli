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
#[serde(deny_unknown_fields)]
pub struct CloudGamesNamespacesCreateGameNamespaceResponse {
	#[serde(rename = "namespace_id")]
	pub namespace_id: uuid::Uuid,
}

impl CloudGamesNamespacesCreateGameNamespaceResponse {
	pub fn new(namespace_id: uuid::Uuid) -> CloudGamesNamespacesCreateGameNamespaceResponse {
		CloudGamesNamespacesCreateGameNamespaceResponse { namespace_id }
	}
}
