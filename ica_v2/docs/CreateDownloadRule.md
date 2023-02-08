# CreateDownloadRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** |  | 
**active** | Option<**bool**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**sequence** | **i32** | Defines the order of the rule. | 
**format_code** | Option<**String**> | Regular expression to filter which format this rule applies to. | [optional]
**project_name** | Option<**String**> | Regular expression to filter which project this rule applies to. | [optional]
**target_local_folder** | **String** | The local folder where to write the data. | 
**file_name_expression** | Option<**String**> | Will allow the filename to be modified including a set of variables | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


