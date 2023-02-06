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
pub struct AuthStartEmailVerificationOutput {
    /// A universally unique identifier.
    #[serde(rename = "verification_id")]
    pub verification_id: String,
}

impl AuthStartEmailVerificationOutput {
    pub fn new(verification_id: String) -> AuthStartEmailVerificationOutput {
        AuthStartEmailVerificationOutput {
            verification_id,
        }
    }
}


