# ActivationCodeDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**allowed_slots** | **i32** | The allowed slot within this code, -1 means unlimited | 
**used_slots** | **i32** | Indicates how many slots can are used. | 
**moved_slots** | **i32** | The slots that where moved to another activation code | 
**original_slots** | **i32** | The assigned allowed slot within this code, -1 means unlimited | 
**pipeline_bundle** | [**crate::models::PipelineBundle**](PipelineBundle.md) |  | 
**usages** | [**Vec<crate::models::ActivationCodeDetailUsage>**](ActivationCodeDetailUsage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


