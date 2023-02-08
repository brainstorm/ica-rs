# DataTransfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**reference** | **String** |  | 
**direction** | **String** |  | 
**connector** | Option<[**crate::models::Connector**](Connector.md)> |  | [optional]
**protocol** | Option<**String**> |  | [optional]
**data_transferred** | **i64** | The data transferred so far in bytes. | 
**status** | **String** |  | 
**status_message** | Option<**String**> | A message explaining the reason why the transfer is in the current status. | [optional]
**duration** | Option<**i64**> | The overall duration of of the transfer defined in seconds. | [optional]
**project** | Option<[**crate::models::Project**](Project.md)> |  | [optional]
**data** | [**crate::models::Data**](Data.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


