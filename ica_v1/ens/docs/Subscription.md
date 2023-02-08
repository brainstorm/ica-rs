# Subscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique id of the subscription | [optional]
**urn** | Option<**String**> | URN of the subscription | [optional]
**_type** | Option<**String**> | Type of event the subscription matches | [optional]
**actions** | Option<**Vec<String>**> | Types of actions the subscription matches for the event type | [optional]
**filter_expression** | Option<**String**> | JSON-structured filter expression for events matching the subscription | [optional]
**name** | Option<**String**> | Name of the subscription | [optional]
**description** | Option<**String**> | Optional description for the subscription | [optional]
**delivery_target** | Option<[**crate::models::DeliveryTarget**](DeliveryTarget.md)> |  | [optional]
**match_identities** | Option<**Vec<String>**> | ACL Identities for events the subscription matches | [optional]
**acl** | Option<**Vec<String>**> | The list of identities that have access to this subscription | [optional]
**tenant_id** | Option<**String**> | Tenant id of the subscription owner | [optional]
**created_by_user_id** | Option<**String**> | User id for the creator of the subscription | [optional]
**time_created** | Option<**String**> | Timestamp when the subscription was created | [optional]
**deleted_by_user_id** | Option<**String**> | Id of the user who deleted the subscription, if applicable | [optional]
**time_deleted** | Option<**String**> | Timestamp when the subscription was deleted, if applicable | [optional]
**is_deleted** | Option<**bool**> | Whether or not the subscription has been deleted | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


