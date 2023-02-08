# BaseConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authenticator** | **String** | Specifies the supported snowflake authenticator to use. Currently 'oauth' only is supported | 
**access_token** | **String** | Specifies the OAuth token to use for authentication | 
**dns_name** | **String** | snowflake dns name. Usually something like '<<account>>.snowflakecomputing.com' | 
**user_principal_name** | **String** | Specifies the user principal name. This is required for some snowflake client (snowSQL for instance) | 
**database_name** | **String** | Specifies the database name bound to the project specified | 
**schema_name** | **String** | Specifies the schema name bound to the project specified | 
**warehouse_name** | **String** | Specifies the warehouse name bound to the project specified | 
**role_name** | **String** | Specifies the role name bound to the project specified | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


