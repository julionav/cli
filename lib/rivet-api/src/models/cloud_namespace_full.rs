/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudNamespaceFull : A full namespace.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudNamespaceFull {
    #[serde(rename = "config")]
    pub config: Box<crate::models::CloudNamespaceConfig>,
    /// RFC3339 timestamp.
    #[serde(rename = "create_ts")]
    pub create_ts: String,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
    #[serde(rename = "name_id")]
    pub name_id: String,
    /// A universally unique identifier.
    #[serde(rename = "namespace_id")]
    pub namespace_id: String,
    /// A universally unique identifier.
    #[serde(rename = "version_id")]
    pub version_id: String,
}

impl CloudNamespaceFull {
    /// A full namespace.
    pub fn new(config: crate::models::CloudNamespaceConfig, create_ts: String, display_name: String, name_id: String, namespace_id: String, version_id: String) -> CloudNamespaceFull {
        CloudNamespaceFull {
            config: Box::new(config),
            create_ts,
            display_name,
            name_id,
            namespace_id,
            version_id,
        }
    }
}


