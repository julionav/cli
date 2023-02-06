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
pub struct ChatTopicDirect {
    #[serde(rename = "identity_a")]
    pub identity_a: Box<crate::models::IdentityHandle>,
    #[serde(rename = "identity_b")]
    pub identity_b: Box<crate::models::IdentityHandle>,
}

impl ChatTopicDirect {
    pub fn new(identity_a: crate::models::IdentityHandle, identity_b: crate::models::IdentityHandle) -> ChatTopicDirect {
        ChatTopicDirect {
            identity_a: Box::new(identity_a),
            identity_b: Box::new(identity_b),
        }
    }
}


