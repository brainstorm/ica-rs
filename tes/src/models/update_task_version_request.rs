/*
 * Task Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateTaskVersionRequest {
    /// User-defined version of task version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// User-defined description of task version
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "execution", skip_serializing_if = "Option::is_none")]
    pub execution: Option<Box<crate::models::Execution>>,
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<String>>,
}

impl UpdateTaskVersionRequest {
    pub fn new() -> UpdateTaskVersionRequest {
        UpdateTaskVersionRequest {
            version: None,
            description: None,
            execution: None,
            acl: None,
        }
    }
}


