/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommonsGameStatSummary : A game statistic summary.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonsGameStatSummary {
    #[serde(rename = "game")]
    pub game: Box<crate::models::CommonsGameHandle>,
    #[serde(rename = "stats")]
    pub stats: Vec<crate::models::CommonsGameStat>,
}

impl CommonsGameStatSummary {
    /// A game statistic summary.
    pub fn new(game: crate::models::CommonsGameHandle, stats: Vec<crate::models::CommonsGameStat>) -> CommonsGameStatSummary {
        CommonsGameStatSummary {
            game: Box::new(game),
            stats,
        }
    }
}


