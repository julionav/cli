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
pub struct IdentityServiceUpdateIdentityProfileRequest {
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "account_number", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<i32>,
    /// Follows regex ^(?:[^\\n\\r]+\\n?|\\n){1,5}$
    #[serde(rename = "bio", skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}

impl IdentityServiceUpdateIdentityProfileRequest {
    pub fn new() -> IdentityServiceUpdateIdentityProfileRequest {
        IdentityServiceUpdateIdentityProfileRequest {
            display_name: None,
            account_number: None,
            bio: None,
        }
    }
}


