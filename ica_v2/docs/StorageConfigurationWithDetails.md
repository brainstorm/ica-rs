# StorageConfigurationWithDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**name** | **String** | The name of the storage configuration | 
**description** | Option<**String**> | An optional description | [optional]
**r#type** | **String** |  | 
**status** | **String** |  | 
**error_message** | Option<**String**> | An optional error message when something went wrong with the configuration | [optional]
**region** | [**crate::models::Region**](Region.md) |  | 
**is_default** | **bool** | An indication if this is the default in region for new projects | 
**storage_configuration_details** | [**crate::models::StorageConfigurationDetails**](StorageConfigurationDetails.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


