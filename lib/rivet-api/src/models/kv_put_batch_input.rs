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
pub struct KvPutBatchInput {
    /// A list of entries to insert.
    #[serde(rename = "entries")]
    pub entries: Vec<crate::models::KvPutEntry>,
    /// A universally unique identifier.
    #[serde(rename = "namespace_id", skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
}

impl KvPutBatchInput {
    pub fn new(entries: Vec<crate::models::KvPutEntry>) -> KvPutBatchInput {
        KvPutBatchInput {
            entries,
            namespace_id: None,
        }
    }
}


