# AwsS3PresignedUrlForUpload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**single_part_url** | Option<**String**> | A single part presigned url for upload | [optional]
**multipart_template** | Option<**String**> | A url template for multi parts presigned url for upload | [optional]
**multipart_signatures** | Option<[**Vec<crate::models::PartInfo>**](PartInfo.md)> | Multi parts info that needs to be applied to the MultipartTemplate | [optional]
**multipart_upload_id** | Option<**String**> | Multi part upload id | [optional]
**server_side_encryption_algorithm** | Option<**String**> | The server side encryption method used by S3.  This value is used to determine the Amazon S3 header \"x-amz-server-side-encryption\" value.  Possible values: 'AES256' and 'aws:kms'. | [optional]
**server_side_encryption_key** | Option<**String**> | Server-side encryption key that might be associated with the specified server-side encryption algorithm  This value can be the AWS KMS arn key, to be used for the Amazon S3 header \"x-amz-server-side-encryption-aws-kms-key-id\" value  This is only used when ServerSideEncryptionAlgorithm is 'aws:kms' | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


