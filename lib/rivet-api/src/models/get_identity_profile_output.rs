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
pub struct GetIdentityProfileOutput {
    #[serde(rename = "identity")]
    pub identity: Box<crate::models::CommonsIdentityProfile>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::CommonsWatchResponse>,
}

impl GetIdentityProfileOutput {
    pub fn new(identity: crate::models::CommonsIdentityProfile, watch: crate::models::CommonsWatchResponse) -> GetIdentityProfileOutput {
        GetIdentityProfileOutput {
            identity: Box::new(identity),
            watch: Box::new(watch),
        }
    }
}


