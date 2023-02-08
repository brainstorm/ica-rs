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
pub struct CreateTaskRequest {
    /// User-defined name of the task
    #[serde(rename = "name")]
    pub name: String,
    /// User-defined description of the task
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Access Control List
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<String>>,
    /// List of task versions
    #[serde(rename = "taskVersions", skip_serializing_if = "Option::is_none")]
    pub task_versions: Option<Vec<crate::models::CreateTaskVersionRequest>>,
}

impl CreateTaskRequest {
    pub fn new(name: String) -> CreateTaskRequest {
        CreateTaskRequest {
            name,
            description: None,
            acl: None,
            task_versions: None,
        }
    }
}