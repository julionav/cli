/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommonsCaptchaConfig : Methods to verify a captcha



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonsCaptchaConfig {
    #[serde(rename = "hcaptcha", skip_serializing_if = "Option::is_none")]
    pub hcaptcha: Option<Box<crate::models::CommonsCaptchaConfigHcaptcha>>,
}

impl CommonsCaptchaConfig {
    /// Methods to verify a captcha
    pub fn new() -> CommonsCaptchaConfig {
        CommonsCaptchaConfig {
            hcaptcha: None,
        }
    }
}


