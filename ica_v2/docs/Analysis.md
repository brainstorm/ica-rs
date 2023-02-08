# Analysis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**reference** | **String** | The unique reference of the analysis | 
**user_reference** | **String** | The user reference of the analysis | 
**pipeline** | [**crate::models::Pipeline**](Pipeline.md) |  | 
**workflow_session** | Option<[**crate::models::WorkflowSession**](WorkflowSession.md)> |  | [optional]
**status** | **String** | The status of the analysis | 
**start_date** | Option<**String**> | When the analysis was started | [optional]
**end_date** | Option<**String**> | When the analysis was finished | [optional]
**summary** | Option<**String**> | The summary of the analysis | [optional]
**analysis_storage** | Option<[**crate::models::AnalysisStorage**](AnalysisStorage.md)> |  | [optional]
**analysis_priority** | Option<**String**> | The priority of the analysis | [optional]
**tags** | [**crate::models::AnalysisTag**](AnalysisTag.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


