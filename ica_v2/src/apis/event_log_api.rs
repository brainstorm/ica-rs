/*
 * ICA Rest API
 *
 * This API can be used to interact with Illumina Connected Analytics.<br> <p> Authentication to the  API can be done in multiple ways:<br> <ul><li>For the entire API, except for the POST /tokens endpoint: API-key + JWT</li> <li>Only for the POST /tokens endpoint: API-key + Basic Authentication</li></ul> </p> <p> <b>API-key</b><br> API keys are managed within the Illumina portal where you can manage your profile after you have logged on. The API-key has to be provided in the X-API-Key header parameter when executing API calls to ICA. In the background, a JWT will be requested at the IDP of Illumina to create a session. A good practice is to not use the API-key for every API call, but to first generate a JWT and to use that for authentication in subsequent calls.<br> </p> <p> <b>JWT</b><br> To avoid using an API-key for each call, we recommend to request a JWT via the POST /tokens endpoint  using this API-key. The JWT will expire after a pre-configured period specified by a tenant administrator through the IAM console in the Illumina portal. The JWT is the preferred way for authentication.<br>A not yet expired, still valid JWT could be refreshed using the POST /tokens:refresh endpoint.<br> </p> <p> <b>Basic Authentication</b><br> Basic authentication is only supported by the POST /tokens endpoint for generating a JWT. Use \"Basic base64encoded(emailaddress:password)\" in the \"Authorization\" header parameter for this authentication method. In case having access to multiple tenants using the same email-address, also provide the \"tenant\" request parameter to indicate what tenant you would like to request a JWT for. </p>
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`get_event_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventLogsError {
    DefaultResponse(crate::models::Problem),
    UnknownValue(serde_json::Value),
}

pub async fn get_event_logs(
    configuration: &configuration::Configuration,
    code: Option<&str>,
    code_filter_type: Option<&str>,
    category: Option<&str>,
    date_from: Option<&str>,
    date_until: Option<&str>,
    rows: Option<i32>,
) -> Result<crate::models::EventLogList, Error<GetEventLogsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/eventLog", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = code {
        local_var_req_builder =
            local_var_req_builder.query(&[("code", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = code_filter_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("codeFilterType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = category {
        local_var_req_builder =
            local_var_req_builder.query(&[("category", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = date_from {
        local_var_req_builder =
            local_var_req_builder.query(&[("dateFrom", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = date_until {
        local_var_req_builder =
            local_var_req_builder.query(&[("dateUntil", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = rows {
        local_var_req_builder =
            local_var_req_builder.query(&[("rows", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Key", local_var_value);
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetEventLogsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
