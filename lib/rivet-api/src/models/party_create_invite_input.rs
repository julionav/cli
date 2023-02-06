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
pub struct PartyCreateInviteInput {
    /// An alias used to join a given party.
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

impl PartyCreateInviteInput {
    pub fn new() -> PartyCreateInviteInput {
        PartyCreateInviteInput {
            alias: None,
        }
    }
}


