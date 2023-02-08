# CreateUploadRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** |  | 
**active** | Option<**bool**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**local_folder** | **String** | The local folder to monitor. Files in this folder on your local environment will be uploaded to the specified project. Only files matching the filePattern will be uploaded. | 
**file_pattern** | **String** | The regular expression to match a file name. eg: to match all files use '.*' | 
**data_format_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The format which will be assigned to the uploaded data. If not specified, an auto-detection of the format will be done. | [optional]
**project_id** | **String** | The project to which the data will be uploaded. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


