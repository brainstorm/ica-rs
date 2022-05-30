# CreateFolderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Folder name, unique to this path, for the folder being created | 
**folder_path** | Option<**String**> | Path from the root folder to the location for the folder being created; must start and end with '/' | [optional]
**volume_id** | Option<**String**> | The unique identifier for this Folder's Volume | [optional]
**volume_name** | Option<**String**> | The unique name for the Folder's Volume | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Metadata about this folder and its contents | [optional]
**acl** | Option<**Vec<String>**> | Optional array to replace the acl on the resource. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


