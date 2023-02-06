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
pub struct CloudGetGroupTransfersListOutput {
    /// The ID of the last transfer listed.
    #[serde(rename = "end_transfer_id", skip_serializing_if = "Option::is_none")]
    pub end_transfer_id: Option<String>,
    /// A list of a group's billing transfers.
    #[serde(rename = "transfers")]
    pub transfers: Vec<crate::models::CloudGroupBillingTransfer>,
}

impl CloudGetGroupTransfersListOutput {
    pub fn new(transfers: Vec<crate::models::CloudGroupBillingTransfer>) -> CloudGetGroupTransfersListOutput {
        CloudGetGroupTransfersListOutput {
            end_transfer_id: None,
            transfers,
        }
    }
}


