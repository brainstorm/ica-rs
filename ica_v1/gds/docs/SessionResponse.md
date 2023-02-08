# SessionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique identifier for this Session | [optional]
**folder_urn** | Option<**String**> | The Universal Resource Name of the Folder associated with the Session | [optional]
**status** | Option<[**crate::models::SessionStatus**](SessionStatus.md)> |  | [optional]
**time_created** | Option<**String**> | The date & time this Session was created, in GDS | [optional]
**time_credentials_expire** | Option<**String**> | The date & time this upload session expires | [optional]
**time_closed** | Option<**String**> | The date & time this Session was closed, in GDS | [optional]
**time_completed** | Option<**String**> | The date & time this Session was completed, in GDS | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


