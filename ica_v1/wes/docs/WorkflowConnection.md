# WorkflowConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**_type** | Option<**String**> | Type of the connection: PlatformJwt, ApiBearerToken, AwsCredentials, ApiCustomAuthentication, BsshOAuthV2 | [optional]
**host** | Option<**String**> | For API based connections, the Host part of the API endpoint | [optional]
**host_validation_regex** | Option<**String**> | Regex that hosts associated with this connection must meet (when host override by each run is allowed) | [optional]
**credentials** | Option<**String**> | Credentials associated with the connection. Format depends on type of connection. | [optional]
**options** | Option<**String**> | Comma separated list of options associated with the connection: CredentialsRequired, AllowCredentialsOverride, AutoDisableCredentialsAfterUse, AllowHostOverride | [optional]
**auto_disable_url** | Option<**String**> | Relative URL (relative to Host) to call in order to disable the credentials | [optional]
**auto_disable_http_method** | Option<**String**> | Http method to use to disable the credentials. Must be POST, PUT or DELETE | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


