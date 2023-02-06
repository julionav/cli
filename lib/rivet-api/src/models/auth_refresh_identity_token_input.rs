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
pub struct AuthRefreshIdentityTokenInput {
    /// When `true`, the current identity for the provided cookie will be logged out and a new identity will be returned.
    #[serde(rename = "logout", skip_serializing_if = "Option::is_none")]
    pub logout: Option<bool>,
}

impl AuthRefreshIdentityTokenInput {
    pub fn new() -> AuthRefreshIdentityTokenInput {
        AuthRefreshIdentityTokenInput {
            logout: None,
        }
    }
}


