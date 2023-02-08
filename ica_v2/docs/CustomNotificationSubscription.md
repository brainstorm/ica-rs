# CustomNotificationSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**custom_event_code** | **String** | The custom event code to subscribe to | 
**filter_expression** | Option<**String**> | To be used when a notification applies to specific conditions. | [optional]
**enabled** | **bool** | Should this subscription be enabled or not? | 
**notification_channel** | [**crate::models::NotificationChannel**](NotificationChannel.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


