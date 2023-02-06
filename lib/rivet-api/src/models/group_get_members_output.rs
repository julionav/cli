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
pub struct GroupGetMembersOutput {
    /// The pagination anchor.
    #[serde(rename = "anchor", skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// A list of group members.
    #[serde(rename = "members")]
    pub members: Vec<crate::models::GroupMember>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::WatchResponse>,
}

impl GroupGetMembersOutput {
    pub fn new(members: Vec<crate::models::GroupMember>, watch: crate::models::WatchResponse) -> GroupGetMembersOutput {
        GroupGetMembersOutput {
            anchor: None,
            members,
            watch: Box::new(watch),
        }
    }
}


