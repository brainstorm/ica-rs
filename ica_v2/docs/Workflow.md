# Workflow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**code** | **String** | The code of the workflow | 
**urn** | **String** | The URN of the workflow. The format is urn:ilmn:ica:\\<type of the object\\>:\\<ID of the object\\>#\\<optional human readable hint representing the object\\>. The hint can be omitted, in that case the hashtag (#) must also be omitted. | 
**description** | **String** | The description of the workflow | 
**language_version** | Option<[**crate::models::PipelineLanguageVersion**](PipelineLanguageVersion.md)> |  | [optional]
**workflow_tags** | Option<[**crate::models::PipelineTag**](PipelineTag.md)> |  | [optional]
**analysis_storage** | [**crate::models::AnalysisStorage**](AnalysisStorage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


