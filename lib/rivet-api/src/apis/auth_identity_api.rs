/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`identity_service_period_complete_email_verification`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdentityServicePeriodCompleteEmailVerificationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`identity_service_period_start_email_verification`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdentityServicePeriodStartEmailVerificationError {
    UnknownValue(serde_json::Value),
}


/// Completes the email verification process.
pub async fn identity_service_period_complete_email_verification(configuration: &configuration::Configuration, auth_complete_email_verification_input: crate::models::AuthCompleteEmailVerificationInput) -> Result<crate::models::AuthCompleteEmailVerificationOutput, Error<IdentityServicePeriodCompleteEmailVerificationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/identity/email/complete-verification", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&auth_complete_email_verification_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IdentityServicePeriodCompleteEmailVerificationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Starts the verification process for linking an emal to your identity.
pub async fn identity_service_period_start_email_verification(configuration: &configuration::Configuration, auth_start_email_verification_input: crate::models::AuthStartEmailVerificationInput) -> Result<crate::models::AuthStartEmailVerificationOutput, Error<IdentityServicePeriodStartEmailVerificationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/identity/email/start-verification", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&auth_start_email_verification_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IdentityServicePeriodStartEmailVerificationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

