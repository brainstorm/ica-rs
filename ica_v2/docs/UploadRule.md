# UploadRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**code** | **String** |  | 
**active** | Option<**bool**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**local_folder** | **String** | The local folder to monitor. Files in this folder on your local environment will be uploaded to the specified project. Only files matching the filePattern will be uploaded. | 
**file_pattern** | **String** | The regular expression to match a file name. eg: to match all files use '.*' | 
**data_format** | Option<[**crate::models::DataFormat**](DataFormat.md)> |  | [optional]
**project** | Option<[**crate::models::Project**](Project.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


