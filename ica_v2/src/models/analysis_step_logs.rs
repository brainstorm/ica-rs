/*
 * ICA Rest API
 *
 * This API can be used to interact with Illumina Connected Analytics.<br> <p> Authentication to the  API can be done in multiple ways:<br> <ul><li>For the entire API, except for the POST /tokens endpoint: API-key + JWT</li> <li>Only for the POST /tokens endpoint: API-key + Basic Authentication</li></ul> </p> <p> <b>API-key</b><br> API keys are managed within the Illumina portal where you can manage your profile after you have logged on. The API-key has to be provided in the X-API-Key header parameter when executing API calls to ICA. In the background, a JWT will be requested at the IDP of Illumina to create a session. A good practice is to not use the API-key for every API call, but to first generate a JWT and to use that for authentication in subsequent calls.<br> </p> <p> <b>JWT</b><br> To avoid using an API-key for each call, we recommend to request a JWT via the POST /tokens endpoint  using this API-key. The JWT will expire after a pre-configured period specified by a tenant administrator through the IAM console in the Illumina portal. The JWT is the preferred way for authentication.<br>A not yet expired, still valid JWT could be refreshed using the POST /tokens:refresh endpoint.<br> </p> <p> <b>Basic Authentication</b><br> Basic authentication is only supported by the POST /tokens endpoint for generating a JWT. Use \"Basic base64encoded(emailaddress:password)\" in the \"Authorization\" header parameter for this authentication method. In case having access to multiple tenants using the same email-address, also provide the \"tenant\" request parameter to indicate what tenant you would like to request a JWT for. </p> 
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AnalysisStepLogs : Contains references to the standard output (stdout) and standard error (stderr) log streams of an analysis step. In this object both log streams could be provided in 2 different formats: as a WebSocket stream URL or as an ICA Data reference. The status of the analysis step determines which format is provided: a WebSocket URL during step execution, a Data reference after step execution. Note however that an analysis step might not expose log streams at all, which would result in this object being empty. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AnalysisStepLogs {
    #[serde(rename = "stdOutData", skip_serializing_if = "Option::is_none")]
    pub std_out_data: Option<Box<crate::models::Data>>,
    /// A WebSocket URL for reading the standard output log stream. Might be closed by ICA as soon as the analysis step execution has finished.
    #[serde(rename = "stdOutStream", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub std_out_stream: Option<Option<String>>,
    #[serde(rename = "stdErrData", skip_serializing_if = "Option::is_none")]
    pub std_err_data: Option<Box<crate::models::Data>>,
    /// A WebSocket URL for reading the standard error log stream. Might be closed by ICA as soon as the analysis step execution has finished.
    #[serde(rename = "stdErrStream", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub std_err_stream: Option<Option<String>>,
}

impl AnalysisStepLogs {
    /// Contains references to the standard output (stdout) and standard error (stderr) log streams of an analysis step. In this object both log streams could be provided in 2 different formats: as a WebSocket stream URL or as an ICA Data reference. The status of the analysis step determines which format is provided: a WebSocket URL during step execution, a Data reference after step execution. Note however that an analysis step might not expose log streams at all, which would result in this object being empty. 
    pub fn new() -> AnalysisStepLogs {
        AnalysisStepLogs {
            std_out_data: None,
            std_out_stream: None,
            std_err_data: None,
            std_err_stream: None,
        }
    }
}

