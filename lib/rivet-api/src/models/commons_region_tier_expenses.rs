/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommonsRegionTierExpenses : Region tier expenses.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonsRegionTierExpenses {
    /// A universally unique identifier.
    #[serde(rename = "namespace_id")]
    pub namespace_id: String,
    /// A universally unique identifier.
    #[serde(rename = "region_id")]
    pub region_id: String,
    /// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
    #[serde(rename = "tier_name_id")]
    pub tier_name_id: String,
    /// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
    #[serde(rename = "lobby_group_name_id")]
    pub lobby_group_name_id: String,
    /// How long a region tier has been active (in milliseconds).
    #[serde(rename = "uptime", skip_serializing_if = "Option::is_none")]
    pub uptime: Option<f64>,
    /// Amount of expenses for this region tier (in hundred-thousandths USD, 100,000 = $1.00).
    #[serde(rename = "expenses", skip_serializing_if = "Option::is_none")]
    pub expenses: Option<f64>,
}

impl CommonsRegionTierExpenses {
    /// Region tier expenses.
    pub fn new(namespace_id: String, region_id: String, tier_name_id: String, lobby_group_name_id: String) -> CommonsRegionTierExpenses {
        CommonsRegionTierExpenses {
            namespace_id,
            region_id,
            tier_name_id,
            lobby_group_name_id,
            uptime: None,
            expenses: None,
        }
    }
}


