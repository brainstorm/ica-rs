# \UserApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approve_user**](UserApi.md#approve_user) | **POST** /api/users/{userId}:approve | Approve a user.
[**assign_tenant_admin_rights_to_user**](UserApi.md#assign_tenant_admin_rights_to_user) | **POST** /api/users/{userId}:assignTenantAdministratorRights | Assign tenant administrator rights to a user.
[**get_user**](UserApi.md#get_user) | **GET** /api/users/{userId} | Retrieve a user.
[**get_users**](UserApi.md#get_users) | **GET** /api/users | Retrieve a list of users.
[**revoke_tenant_admin_rights_to_user**](UserApi.md#revoke_tenant_admin_rights_to_user) | **POST** /api/users/{userId}:revokeTenantAdministratorRights | Revoke tenant administrator rights to a user.
[**update_user**](UserApi.md#update_user) | **PUT** /api/users/{userId} | Update a user.



## approve_user

> approve_user(user_id)
Approve a user.

Endpoint for approving a user.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_tenant_admin_rights_to_user

> assign_tenant_admin_rights_to_user(user_id)
Assign tenant administrator rights to a user.

Endpoint for assigning tenant administrator rights to a user.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::User get_user(user_id)
Retrieve a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> crate::models::UserList get_users(email_address)
Retrieve a list of users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address** | Option<**String**> | The email address to filter on |  |

### Return type

[**crate::models::UserList**](UserList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_tenant_admin_rights_to_user

> revoke_tenant_admin_rights_to_user(user_id)
Revoke tenant administrator rights to a user.

Endpoint for revoking tenant administrator rights to a user.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::User update_user(user_id, if_match, user)
Update a user.

Fields which can be updated: - greeting - two factor authentication - job title - first name - last name - mobile phone number - phone number - fax number - address lines - postal code - city - country - state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**if_match** | Option<**String**> | Optional header parameter to enable conflict exposure. If the client provides this header, then it must contains the client's most recent value of the 'ETag' response header, and the server will respond with a 409 code if it detects a conflict. If the client does not provide this header, then the server will not do a conflict check, which means that as a client you can override the resource even when the server has a more recent version. |  |
**user** | Option<[**User**](User.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

