/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JobResponse {
    /// A unique identifier for this Job
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The Universal Resource Name of the parent folder associated with the Job
    #[serde(rename = "parentFolderUrn", skip_serializing_if = "Option::is_none")]
    pub parent_folder_urn: Option<String>,
    #[serde(rename = "operationType", skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<crate::models::JobOperationType>,
    #[serde(
        rename = "operationParameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_parameters: Option<Box<crate::models::JobOperationParameters>>,
    #[serde(rename = "progressStatus", skip_serializing_if = "Option::is_none")]
    pub progress_status: Option<crate::models::JobProgressStatus>,
    /// The date & time this Folder was created, in GDS
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    /// The creator of this Job
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The date & time this Job was updated, in GDS
    #[serde(rename = "timeModified", skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    /// The date & time this Job was completed, in GDS
    #[serde(rename = "timeCompleted", skip_serializing_if = "Option::is_none")]
    pub time_completed: Option<String>,
}

impl JobResponse {
    pub fn new() -> JobResponse {
        JobResponse {
            id: None,
            parent_folder_urn: None,
            operation_type: None,
            operation_parameters: None,
            progress_status: None,
            time_created: None,
            created_by: None,
            time_modified: None,
            time_completed: None,
        }
    }
}