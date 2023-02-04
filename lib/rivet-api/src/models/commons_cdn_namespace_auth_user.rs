/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommonsCdnNamespaceAuthUser : An authenticated CDN user for a given namespace.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonsCdnNamespaceAuthUser {
    /// A user name.
    #[serde(rename = "user")]
    pub user: String,
}

impl CommonsCdnNamespaceAuthUser {
    /// An authenticated CDN user for a given namespace.
    pub fn new(user: String) -> CommonsCdnNamespaceAuthUser {
        CommonsCdnNamespaceAuthUser {
            user,
        }
    }
}


