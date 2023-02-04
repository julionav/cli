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
pub struct PortalGetGameProfileOutput {
    #[serde(rename = "game")]
    pub game: Box<crate::models::CommonsGameProfile>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::CommonsWatchResponse>,
}

impl PortalGetGameProfileOutput {
    pub fn new(game: crate::models::CommonsGameProfile, watch: crate::models::CommonsWatchResponse) -> PortalGetGameProfileOutput {
        PortalGetGameProfileOutput {
            game: Box::new(game),
            watch: Box::new(watch),
        }
    }
}


