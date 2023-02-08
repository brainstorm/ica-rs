# ProjectPermission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**role_project** | **String** |  | 
**role_flow** | **String** |  | 
**role_base** | **String** |  | 
**role_bench** | **String** |  | 
**membership_type** | **String** |  | 
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**email_address** | Option<**String**> | Only present when membershipType is EMAIL | [optional]
**workgroup** | Option<[**crate::models::Workgroup**](Workgroup.md)> |  | [optional]
**invitation_accepted** | Option<**bool**> | Only present when membershipType is EMAIL | [optional]
**invitation_rejected** | Option<**bool**> | Only present when user is invited by EMAIL | [optional]
**upload_allowed** | **bool** |  | 
**download_allowed** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


