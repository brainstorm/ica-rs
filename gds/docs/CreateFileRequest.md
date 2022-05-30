# CreateFileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the file to be uploaded, case sensitive. | 
**volume_id** | Option<**String**> | Volume ID to which the file will be uploaded | [optional]
**folder_path** | Option<**String**> | Optional folder path where the file will be uploaded to | [optional]
**_type** | Option<**String**> | Optional file content type, e.g. text/plain, application/json | [optional]
**volume_name** | Option<**String**> | Name of the Volume | [optional]
**format** | Option<**String**> | The File's Format | [optional]
**format_edam** | Option<**String**> | The File's Edam Format | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Metadata about this file and its contents | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


