# ProjectBaseTable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**name** | **String** | The name of the table which should be used in writing queries | 
**description** | Option<**String**> | The description of the table | [optional]
**r#type** | **String** | The type of the table | 
**status** | **String** | The status of the table | 
**number_of_records** | Option<**i64**> | The number of record in the table | [optional]
**data_size** | Option<**i64**> | The amount of Data contained in this table in bytes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


