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
pub struct PartyJoinPartyOutput {
    /// A universally unique identifier.
    #[serde(rename = "party_id")]
    pub party_id: String,
}

impl PartyJoinPartyOutput {
    pub fn new(party_id: String) -> PartyJoinPartyOutput {
        PartyJoinPartyOutput {
            party_id,
        }
    }
}


