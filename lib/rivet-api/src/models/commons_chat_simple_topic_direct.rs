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
pub struct CommonsChatSimpleTopicDirect {
    #[serde(rename = "identity_a")]
    pub identity_a: uuid::Uuid,
    #[serde(rename = "identity_b")]
    pub identity_b: uuid::Uuid,
}

impl CommonsChatSimpleTopicDirect {
    pub fn new(identity_a: uuid::Uuid, identity_b: uuid::Uuid) -> CommonsChatSimpleTopicDirect {
        CommonsChatSimpleTopicDirect {
            identity_a,
            identity_b,
        }
    }
}


