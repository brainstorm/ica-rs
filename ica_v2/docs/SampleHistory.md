# SampleHistory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**occurred_at** | **String** | When the change was made | 
**user** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The user that made the change | [optional]
**run** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | In which execution context the change was made | [optional]
**source** | **String** | In which context the change was made | 
**text** | **String** | What was changed | 
**project** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | In which project context the change was made | [optional]
**model** | Option<**i64**> | In which model context the change was made | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


