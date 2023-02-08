# Connector

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**code** | **String** |  | 
**active** | **bool** |  | 
**connected** | **bool** | Indicates if the connector is connected or not. This is cached so even when the connector is no longer connected, for a short time this still may return true. | 
**technical_code** | **String** | Technical code to be used for processing. | 
**initialization_key** | Option<**String**> | The key provided via other channels to initialize the installation. | [optional]
**country** | Option<[**crate::models::Country**](Country.md)> |  | [optional]
**address_line1** | Option<**String**> |  | [optional]
**address_line2** | Option<**String**> |  | [optional]
**address_line3** | Option<**String**> |  | [optional]
**postal_code** | Option<**String**> |  | [optional]
**city** | Option<**String**> |  | [optional]
**state** | Option<**String**> |  | [optional]
**description** | Option<**String**> | The general description of the connector instance including its purpose. | [optional]
**mode** | **String** | The mode the connector runs in. | 
**max_bandwidth** | Option<**f32**> | The maximum bandwidth defined in MB per second. | [optional]
**max_concurrent_transfers** | Option<**i32**> | The maximum amount of concurrent transfers that this connector can execute. | [optional]
**os** | **String** | The target OS of the original connector installer. | 
**installation_status** | **String** |  | 
**new_connector_version_available** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


