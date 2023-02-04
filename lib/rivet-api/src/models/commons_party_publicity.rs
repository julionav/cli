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
pub struct CommonsPartyPublicity {
    #[serde(rename = "public")]
    pub public: crate::models::CommonsPartyPublicityLevel,
    #[serde(rename = "mutual_followers")]
    pub mutual_followers: crate::models::CommonsPartyPublicityLevel,
    #[serde(rename = "groups")]
    pub groups: crate::models::CommonsPartyPublicityLevel,
}

impl CommonsPartyPublicity {
    pub fn new(public: crate::models::CommonsPartyPublicityLevel, mutual_followers: crate::models::CommonsPartyPublicityLevel, groups: crate::models::CommonsPartyPublicityLevel) -> CommonsPartyPublicity {
        CommonsPartyPublicity {
            public,
            mutual_followers,
            groups,
        }
    }
}


