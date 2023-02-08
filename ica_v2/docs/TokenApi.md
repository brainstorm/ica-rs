# \TokenApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_jwt_token**](TokenApi.md#create_jwt_token) | **POST** /api/tokens | Generate a JWT using an API-key, Basic Authentication or a psToken.
[**refresh_jwt_token**](TokenApi.md#refresh_jwt_token) | **POST** /api/tokens:refresh | Refresh a JWT using a not yet expired, still valid JWT.



## create_jwt_token

> crate::models::Token create_jwt_token(tenant)
Generate a JWT using an API-key, Basic Authentication or a psToken.

Generate a JWT using an API-key, Basic Authentication or a psToken. When using Basic Authentication, and you are member of several tenants, also provide the tenant request parameter to indicate for which tenant you want to authenticate. Note that Basic Authentication will not work for SSO (Single Sign On) enabled authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | Option<**String**> | The name of your tenant in case you have access to multiple tenants. |  |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BasicAuth](../README.md#BasicAuth), [PsTokenAuth](../README.md#PsTokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_jwt_token

> crate::models::Token refresh_jwt_token()
Refresh a JWT using a not yet expired, still valid JWT.

When still having a valid JWT, this endpoint can be used to extend the validity.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

