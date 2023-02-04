/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommonsGameLobbyExpenses : Game lobby expenses.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonsGameLobbyExpenses {
    #[serde(rename = "game")]
    pub game: Box<crate::models::CommonsGameHandle>,
    /// A list of namespace summaries.
    #[serde(rename = "namespaces")]
    pub namespaces: Vec<crate::models::CommonsNamespaceSummary>,
    /// A list of multiple region tier expenses.
    #[serde(rename = "expenses")]
    pub expenses: Vec<crate::models::CommonsRegionTierExpenses>,
}

impl CommonsGameLobbyExpenses {
    /// Game lobby expenses.
    pub fn new(game: crate::models::CommonsGameHandle, namespaces: Vec<crate::models::CommonsNamespaceSummary>, expenses: Vec<crate::models::CommonsRegionTierExpenses>) -> CommonsGameLobbyExpenses {
        CommonsGameLobbyExpenses {
            game: Box::new(game),
            namespaces,
            expenses,
        }
    }
}


