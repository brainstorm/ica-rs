# Pipeline

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**code** | **String** | The code of the pipeline | 
**urn** | Option<**String**> | The URN of the pipeline. The format is urn:ilmn:ica:\\<type of the object\\>:\\<ID of the object\\>#\\<optional human readable hint representing the object\\>. The hint can be omitted, in that case the hashtag (#) must also be omitted. | [optional]
**description** | **String** | The description of the pipeline | 
**language** | **String** | The language that is used by the pipeline | 
**language_version** | Option<[**crate::models::PipelineLanguageVersion**](PipelineLanguageVersion.md)> |  | [optional]
**pipeline_tags** | [**crate::models::PipelineTag**](PipelineTag.md) |  | 
**analysis_storage** | [**crate::models::AnalysisStorage**](AnalysisStorage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


