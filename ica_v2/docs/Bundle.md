# Bundle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**short_description** | Option<**String**> |  | [optional]
**region** | [**crate::models::Region**](Region.md) |  | 
**metadata_model** | Option<[**crate::models::MetadataModel**](MetadataModel.md)> |  | [optional]
**release_version** | **String** |  | 
**version_comment** | Option<**String**> |  | [optional]
**status** | **String** |  | 
**categories** | Option<**Vec<String>**> | category tags as string array | [optional]
**links** | Option<[**crate::models::Links**](Links.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


