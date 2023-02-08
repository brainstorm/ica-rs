# BaseJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**description** | Option<**String**> | A short description of the base job | [optional]
**table** | Option<[**crate::models::ProjectBaseTable**](ProjectBaseTable.md)> |  | [optional]
**r#type** | **String** | The type of the job | 
**status** | **String** | The status of the job | 
**overall_duration** | Option<**i64**> | The duration of the job expressed in milliseconds | [optional]
**details** | Option<**String**> | Detailed description of the job | [optional]
**bytes_billed** | Option<**i64**> | Bytes billed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


