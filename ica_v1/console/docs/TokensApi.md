# \TokensApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_token**](TokensApi.md#create_token) | **POST** /v1/tokens | Creates a JWT token to call IAP services.
[**get_token_details**](TokensApi.md#get_token_details) | **GET** /v1/tokens/details | Get current tokens info require authorization Bearer token
[**refresh_token**](TokensApi.md#refresh_token) | **POST** /v1/tokens:refresh | Refresh session psToken.
[**revoke_token**](TokensApi.md#revoke_token) | **DELETE** /v1/tokens | Revokes an access token.



## create_token

> crate::models::TokenResponse create_token(x_api_key, client_id, api_key, domain, data, scopes, cwid, cid, return_session_token)
Creates a JWT token to call IAP services.

This endpoint creates a JWT token to call IAP services. Authorization can be a Bearer psToken,  Basic Base64 encoded username:password or Basic with apiKey.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | Option<**String**> | Api Key can be passed in header to generate a JWT. |  |
**client_id** | Option<**String**> | Optionally pass client Id from calling app to set as authorized party on JWT. |  |
**api_key** | Option<**String**> | OBSOLETE: api key should now be passed as as an X-API-Key header. |  |
**domain** | Option<**String**> | Optionally pass the domain name you are logging into |  |
**data** | Option<**String**> | Data is a custom meta data field that will be applied to the session field in the JWT payload. |  |
**scopes** | Option<[**Vec<String>**](String.md)> | Scopes can be passed in during token generation to limit the token to particular scopes. |  |
**cwid** | Option<**String**> | Set the current workgroup on the token. Used for aligning resources to a workgroup. |  |
**cid** | Option<**String**> | Set the current context on the token. Used for aligning resources to a context. |  |
**return_session_token** | Option<**bool**> | By default, this endpoint returns a JWT token. You can specify returnSessionToken=true to get an Illumina psToken instead. |  |

### Return type

[**crate::models::TokenResponse**](TokenResponse.md)

### Authorization

[Basic](../README.md#Basic), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_token_details

> crate::models::TokenDetailResponse get_token_details()
Get current tokens info require authorization Bearer token

Get token details

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TokenDetailResponse**](TokenDetailResponse.md)

### Authorization

[Basic](../README.md#Basic), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_token

> crate::models::TokenResponse refresh_token(body)
Refresh session psToken.

This endpoint extends the session for the psToken.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**AccessTokenRequest**](AccessTokenRequest.md)> | Access token request accepts a psToken in the access_token field in the body of the request. |  |

### Return type

[**crate::models::TokenResponse**](TokenResponse.md)

### Authorization

[Basic](../README.md#Basic), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_token

> revoke_token(body)
Revokes an access token.

This endpoint revokes the access token that is passed in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**AccessTokenRequest**](AccessTokenRequest.md)> | Access token request accepts either a psToken or a JWT in the access_token field in the body of the request. |  |

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

