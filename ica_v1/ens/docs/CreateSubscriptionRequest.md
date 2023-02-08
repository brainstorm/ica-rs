# CreateSubscriptionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | Event type which will be subscribed to | 
**actions** | Option<**Vec<String>**> | Actions which will be subscribed to for the event type | [optional]
**name** | **String** | Name of the subscription | 
**description** | Option<**String**> | Optional description for the subscription | [optional]
**filter_expression** | Option<**String**> | JSON-structured filter expression for events matching the subscription | [optional]
**delivery_target** | [**crate::models::DeliveryTarget**](DeliveryTarget.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


