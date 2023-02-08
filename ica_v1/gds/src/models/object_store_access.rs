/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// ObjectStoreAccess : ObjectStoreAccess to get the temporaryCredentials per provider

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjectStoreAccess {
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(
        rename = "awsS3TemporaryUploadCredentials",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_s3_temporary_upload_credentials:
        Option<Box<crate::models::AwsS3TemporaryUploadCredentials>>,
    #[serde(
        rename = "awsS3PresignedUrlForUpload",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_s3_presigned_url_for_upload: Option<Box<crate::models::AwsS3PresignedUrlForUpload>>,
}

impl ObjectStoreAccess {
    /// ObjectStoreAccess to get the temporaryCredentials per provider
    pub fn new() -> ObjectStoreAccess {
        ObjectStoreAccess {
            session_id: None,
            aws_s3_temporary_upload_credentials: None,
            aws_s3_presigned_url_for_upload: None,
        }
    }
}