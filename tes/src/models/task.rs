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
pub struct Task {
    /// Global identifier for object
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Href of the object
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// URN of the resource
    #[serde(rename = "urn", skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "taskVersions", skip_serializing_if = "Option::is_none")]
    pub task_versions: Option<Vec<crate::models::TaskVersion>>,
    /// Access Control List
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<String>>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "subTenantId", skip_serializing_if = "Option::is_none")]
    pub sub_tenant_id: Option<String>,
    /// User who created the object
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Date and Time (in UTC) when object was created in TES
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    /// User who updated the object
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    /// Date and Time (in UTC) when object was modified in TES
    #[serde(rename = "timeModified", skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
}

impl Task {
    pub fn new() -> Task {
        Task {
            id: None,
            href: None,
            urn: None,
            name: None,
            description: None,
            task_versions: None,
            acl: None,
            tenant_id: None,
            sub_tenant_id: None,
            created_by: None,
            time_created: None,
            modified_by: None,
            time_modified: None,
        }
    }
}


