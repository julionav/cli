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
pub struct CommonsChatThreadExternalLinks {
    #[serde(rename = "chat")]
    pub chat: String,
}

impl CommonsChatThreadExternalLinks {
    pub fn new(chat: String) -> CommonsChatThreadExternalLinks {
        CommonsChatThreadExternalLinks {
            chat,
        }
    }
}


