# Awss3ObjectStoreSetting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bucket_name** | **String** | The bucket name | 
**key_prefix** | Option<**String**> | Key prefix within the bucket for GDS to operate within. Volumes may only be created within this prefix and the given credentials need only authorize  access here. If not set, default is to allow operation on the full bucket. No leading slash, and must end with a trailing slash. | [optional]
**server_side_encryption_algorithm** | Option<**String**> | Used to specify the type of server-side encryption (SSE) to be used on the object provider.  This value is used to determine the Amazon S3 header \"x-amz-server-side-encryption\" value.  For example, specify \"AES256\" for SSE-S3, or \"AWS:KMS\" for SSE-KMS.  By default if none is specified, \"AES256\" will be used. | [optional]
**server_side_encryption_key** | Option<**String**> | Used to specify the serve-side encryption key that might be associated with the specified server-side encryption algorithm  This value can be the AWS KMS arn key, to be used for the Amazon S3 header \"x-amz-server-side-encryption-aws-kms-key-id\" value  Value will be ignored if encryption is \"AES256\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


