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
pub struct CommonsChatSimpleTopicParty {
    #[serde(rename = "party")]
    pub party: uuid::Uuid,
}

impl CommonsChatSimpleTopicParty {
    pub fn new(party: uuid::Uuid) -> CommonsChatSimpleTopicParty {
        CommonsChatSimpleTopicParty {
            party,
        }
    }
}


