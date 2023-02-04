/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommonsValidationError : An error given by failed content validation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonsValidationError {
    /// A list of strings denoting the origin of a validation error.
    #[serde(rename = "path")]
    pub path: Vec<String>,
}

impl CommonsValidationError {
    /// An error given by failed content validation.
    pub fn new(path: Vec<String>) -> CommonsValidationError {
        CommonsValidationError {
            path,
        }
    }
}


