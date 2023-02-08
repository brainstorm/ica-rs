# NotificationChannel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**enabled** | **bool** | Should this channel be enabled or not? | 
**r#type** | **String** | The type of delivery target (MAIL, SQS, SNS, HTTP, ...) | 
**address** | **String** | The address where to send a notification to (email address, url, ...) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


