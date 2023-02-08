/*
 * ICA Rest API
 *
 * This API can be used to interact with Illumina Connected Analytics.<br> <p> Authentication to the  API can be done in multiple ways:<br> <ul><li>For the entire API, except for the POST /tokens endpoint: API-key + JWT</li> <li>Only for the POST /tokens endpoint: API-key + Basic Authentication</li></ul> </p> <p> <b>API-key</b><br> API keys are managed within the Illumina portal where you can manage your profile after you have logged on. The API-key has to be provided in the X-API-Key header parameter when executing API calls to ICA. In the background, a JWT will be requested at the IDP of Illumina to create a session. A good practice is to not use the API-key for every API call, but to first generate a JWT and to use that for authentication in subsequent calls.<br> </p> <p> <b>JWT</b><br> To avoid using an API-key for each call, we recommend to request a JWT via the POST /tokens endpoint  using this API-key. The JWT will expire after a pre-configured period specified by a tenant administrator through the IAM console in the Illumina portal. The JWT is the preferred way for authentication.<br>A not yet expired, still valid JWT could be refreshed using the POST /tokens:refresh endpoint.<br> </p> <p> <b>Basic Authentication</b><br> Basic authentication is only supported by the POST /tokens endpoint for generating a JWT. Use \"Basic base64encoded(emailaddress:password)\" in the \"Authorization\" header parameter for this authentication method. In case having access to multiple tenants using the same email-address, also provide the \"tenant\" request parameter to indicate what tenant you would like to request a JWT for. </p> 
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CwlAnalysisJsonInput {
    #[serde(rename = "objectType")]
    pub object_type: ObjectType,
    /// Contains the input JSON, as an escaped JSON String.
    #[serde(rename = "inputJson")]
    pub input_json: String,
    #[serde(rename = "dataIds", skip_serializing_if = "Option::is_none")]
    pub data_ids: Option<Vec<String>>,
    #[serde(rename = "mounts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Option<Vec<crate::models::AnalysisInputDataMount>>>,
}

impl CwlAnalysisJsonInput {
    pub fn new(object_type: ObjectType, input_json: String) -> CwlAnalysisJsonInput {
        CwlAnalysisJsonInput {
            object_type,
            input_json,
            data_ids: None,
            mounts: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ObjectType {
    #[serde(rename = "STRUCTURED")]
    Structured,
    #[serde(rename = "JSON")]
    Json,
}

impl Default for ObjectType {
    fn default() -> ObjectType {
        Self::Structured
    }
}
