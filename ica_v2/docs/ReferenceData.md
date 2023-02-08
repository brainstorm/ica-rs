# ReferenceData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**name** | **String** | The name of the reference data | 
**species** | Option<[**crate::models::Species**](Species.md)> |  | [optional]
**data_format** | Option<[**crate::models::DataFormat**](DataFormat.md)> |  | [optional]
**version** | **String** | The version of the reference data | 
**type_list** | [**crate::models::TypeList**](TypeList.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


