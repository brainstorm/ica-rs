# WorkflowSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**user_reference** | **String** | The user reference of the workflow session | 
**workflow** | [**crate::models::Workflow**](Workflow.md) |  | 
**status** | **String** | The status of the workflow session | 
**start_date** | Option<**String**> | When the workflow session was started | [optional]
**end_date** | Option<**String**> | When the workflow session was finished | [optional]
**summary** | Option<**String**> | The summary of the workflow session | [optional]
**tags** | [**crate::models::WorkflowSessionTag**](WorkflowSessionTag.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


