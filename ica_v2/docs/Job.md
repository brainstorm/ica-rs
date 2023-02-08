# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**status** | **String** |  | 
**additional_status_information** | Option<**String**> | Additional information regarding the status of this job. | [optional]
**subject_type** | **String** | The type of the subject for which this job provides execution. | 
**subject_id** | [**uuid::Uuid**](uuid::Uuid.md) | The id of the subject for which this job provides execution. | 
**time_created** | **String** |  | 
**time_started** | Option<**String**> |  | [optional]
**time_finished** | Option<**String**> |  | [optional]
**owner** | [**crate::models::User**](User.md) |  | 
**project** | Option<[**crate::models::Project**](Project.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


