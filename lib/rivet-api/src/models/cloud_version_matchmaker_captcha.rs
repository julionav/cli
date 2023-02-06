/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerCaptcha : Matchmaker captcha configuration.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionMatchmakerCaptcha {
    #[serde(rename = "hcaptcha", skip_serializing_if = "Option::is_none")]
    pub hcaptcha: Option<Box<crate::models::CloudVersionMatchmakerCaptchaHcaptcha>>,
    /// Denotes how many requests a connection can make before it is required to reverify a captcha.
    #[serde(rename = "requests_before_reverify")]
    pub requests_before_reverify: i32,
    /// Denotes how long a connection can continue to reconnect without having to reverify a captcha (in milliseconds).
    #[serde(rename = "verification_ttl")]
    pub verification_ttl: i64,
}

impl CloudVersionMatchmakerCaptcha {
    /// Matchmaker captcha configuration.
    pub fn new(requests_before_reverify: i32, verification_ttl: i64) -> CloudVersionMatchmakerCaptcha {
        CloudVersionMatchmakerCaptcha {
            hcaptcha: None,
            requests_before_reverify,
            verification_ttl,
        }
    }
}


