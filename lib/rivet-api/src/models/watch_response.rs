/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WatchResponse : Provided by watchable endpoints used in blocking loops.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WatchResponse {
    /// Index indicating the version of the data responded. Pass this to `WatchQuery` to block and wait for the next response. 
    #[serde(rename = "index")]
    pub index: String,
}

impl WatchResponse {
    /// Provided by watchable endpoints used in blocking loops.
    pub fn new(index: String) -> WatchResponse {
        WatchResponse {
            index,
        }
    }
}


