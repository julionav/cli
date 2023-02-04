/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityUpdateIdentityGameActivity : Information about the identity's current game. This is information that all other identities can see about what the current identity is doing.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityUpdateIdentityGameActivity {
    /// A short message about the current game activity.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// JSON data seen by anyone.
    #[serde(rename = "public_metadata", deserialize_with = "Option::deserialize")]
    pub public_metadata: Option<serde_json::Value>,
    /// JSON data seen only by the given identity and their mutual followers.
    #[serde(rename = "mutual_metadata", deserialize_with = "Option::deserialize")]
    pub mutual_metadata: Option<serde_json::Value>,
}

impl IdentityUpdateIdentityGameActivity {
    /// Information about the identity's current game. This is information that all other identities can see about what the current identity is doing.
    pub fn new(public_metadata: Option<serde_json::Value>, mutual_metadata: Option<serde_json::Value>) -> IdentityUpdateIdentityGameActivity {
        IdentityUpdateIdentityGameActivity {
            message: None,
            public_metadata,
            mutual_metadata,
        }
    }
}


