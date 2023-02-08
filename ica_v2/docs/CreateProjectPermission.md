# CreateProjectPermission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**role_project** | **String** |  | 
**role_flow** | **String** |  | 
**role_base** | **String** |  | 
**role_bench** | **String** |  | 
**membership_type** | **String** | How users are invited to the project | 
**user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | the id of the user that should be given access, required when membershipType is USER | [optional]
**email_address** | Option<**String**> | The email to invite a user on, required when membershipType is EMAIL | [optional]
**workgroup_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | the id of the workgroup to give access, required when membershipType is WORKGROUP | [optional]
**upload_allowed** | **bool** | Indicates if uploading data is allowed or not. | 
**download_allowed** | **bool** | Indicates if downloading data is allowed or not. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


