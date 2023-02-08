# CreateStorageConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the configuration | 
**description** | Option<**String**> | An optional description | [optional]
**storage_credential_id** | [**uuid::Uuid**](uuid::Uuid.md) | The id of the storage credential | 
**r#type** | **String** | The type of configuration | 
**aws_details** | Option<[**crate::models::AwsDetails**](AWSDetails.md)> |  | [optional]
**region_id** | [**uuid::Uuid**](uuid::Uuid.md) | The id of the region where the bucket will be located | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


