# Execution

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image** | Option<[**crate::models::Image**](Image.md)> |  | [optional]
**command** | Option<**String**> |  | [optional]
**args** | Option<**Vec<String>**> | Argument to run specified task | [optional]
**inputs** | Option<[**Vec<crate::models::InputMountMappingWithCreds>**](InputMountMappingWithCreds.md)> | Path (Inputs) - Path to mount file at valid Url  URL (Inputs) - Url of file mounted at specified path | [optional]
**outputs** | Option<[**Vec<crate::models::MountMappingWithCreds>**](MountMappingWithCreds.md)> | Path (Outputs) - Path where files will be output to valid Url  URL (Outputs) - Url of folder with files from the path will be uploaded | [optional]
**system_files** | Option<[**crate::models::SystemFiles**](SystemFiles.md)> |  | [optional]
**environment** | Option<[**crate::models::Environment**](Environment.md)> |  | [optional]
**working_directory** | Option<**String**> |  | [optional]
**retry_limit** | Option<**i32**> |  | [optional][default to 3]
**retry_count** | Option<**i32**> |  | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


