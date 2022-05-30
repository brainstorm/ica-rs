# AwsS3TemporaryUploadCredentials

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key_id** | Option<**String**> | Access key for use with AWS S3 | [optional]
**secret_access_key** | Option<**String**> | Secret key for use with AWS S3 | [optional]
**session_token** | Option<**String**> | Token for use with AWS S3 | [optional]
**region** | Option<**String**> | AWS region the folder will/does reside in | [optional]
**bucket_name** | Option<**String**> | AWS bucket the folder will/does reside in | [optional]
**key_prefix** | Option<**String**> | AWS upload location for this folder | [optional]
**expiration_date** | Option<**String**> | expiration for temporary credentials | [optional]
**service_url** | Option<**String**> | Service endpoint for accessing S3.  This is optional for AWS S3, but mandatory for other services like Taiwan Computing Cloud. | [optional]
**server_side_encryption_algorithm** | Option<**String**> | Used to specify the type of server-side encryption (SSE) to be used on the object provider.  This value is used to determine the Amazon S3 header \"x-amz-server-side-encryption\" value.  For example, specify \"AES256\" for SSE-S3, or \"AWS:KMS\" for SSE-KMS.  By default if none is specified, \"AES256\" will be used. | [optional]
**server_side_encryption_key** | Option<**String**> | Used to specify the serve-side encryption key that might be associated with the specified server-side encryption algorithm  This value can be the AWS KMS arn key, to be used for the Amazon S3 header \"x-amz-server-side-encryption-aws-kms-key-id\" value  Value will be ignored if encryption is \"AES256\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


