# CreateData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the file/folder as how it will be created. | 
**folder_id** | Option<**String**> | The id of the folder you want to create this new data in. Alternatively, the folderPath attribute could be used as well for this. | [optional]
**folder_path** | Option<**String**> | The absolute path of the folder you want to create this new data in which must end with '/'. Alternatively, the folderId attribute could be used as well for this. In case the folder path does not yet exist, it will be automatically created. | [optional]
**format_code** | Option<**String**> | The code of the format you would like to assign at creation time. This is only allowed for file data. If not specified, auto format assignment will be done. | [optional]
**data_type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


