# CreateConnector

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** |  | 
**active** | **bool** |  | 
**country_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | ID of the country. If not provided then the country of the tenant will be used. | [optional]
**address_line1** | Option<**String**> |  | [optional]
**address_line2** | Option<**String**> |  | [optional]
**address_line3** | Option<**String**> |  | [optional]
**postal_code** | Option<**String**> |  | [optional]
**city** | Option<**String**> |  | [optional]
**state** | Option<**String**> |  | [optional]
**description** | Option<**String**> | The general description of the connector instance including its purpose. | [optional]
**mode** | **String** | The mode the connector runs in. | 
**max_bandwidth** | Option<**f32**> | The maximum bandwidth defined in MB per second. | [optional]
**max_concurrent_transfers** | Option<**i32**> | The maximum amount of concurrent transfers that this connector can execute. | [optional][default to 2]
**os** | **String** | The target OS of the original connector installer. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


