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
pub struct ChatMessage {
    #[serde(rename = "chat_message_id")]
    pub chat_message_id: uuid::Uuid,
    /// RFC3339 timestamp
    #[serde(rename = "send_ts")]
    pub send_ts: String,
    #[serde(rename = "thread_id")]
    pub thread_id: uuid::Uuid,
}

impl ChatMessage {
    pub fn new(chat_message_id: uuid::Uuid, send_ts: String, thread_id: uuid::Uuid) -> ChatMessage {
        ChatMessage {
            chat_message_id,
            send_ts,
            thread_id,
        }
    }
}


