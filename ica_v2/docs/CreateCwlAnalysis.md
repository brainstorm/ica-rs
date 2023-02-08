# CreateCwlAnalysis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_reference** | **String** | The user-reference of the analysis. This should be something meaningful for the user. | 
**pipeline_id** | **String** | The pipeline for which an analysis will be created. | 
**tags** | [**crate::models::AnalysisTag**](AnalysisTag.md) |  | 
**activation_code_detail_id** | [**uuid::Uuid**](uuid::Uuid.md) | Indicates under which activation code the pipeline is executed. | 
**analysis_storage_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The id of the storage to use for the analysis. | [optional]
**output_parent_folder_id** | Option<**String**> | The id of the folder in which the output folder should be created. | [optional]
**analysis_output** | Option<[**Vec<crate::models::AnalysisOutputMapping>**](AnalysisOutputMapping.md)> |  | [optional]
**analysis_input** | [**crate::models::CwlAnalysisInput**](CwlAnalysisInput.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


