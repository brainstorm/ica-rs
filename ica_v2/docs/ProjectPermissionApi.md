# \ProjectPermissionApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_permission**](ProjectPermissionApi.md#create_project_permission) | **POST** /api/projects/{projectId}/permissions | Create a project permission.
[**get_project_permission**](ProjectPermissionApi.md#get_project_permission) | **GET** /api/projects/{projectId}/permissions/{permissionId} | Retrieve a project permission.
[**get_project_permissions**](ProjectPermissionApi.md#get_project_permissions) | **GET** /api/projects/{projectId}/permissions | Retrieve a list of project permissions.
[**update_project_permission**](ProjectPermissionApi.md#update_project_permission) | **PUT** /api/projects/{projectId}/permissions/{permissionId} | Update a project permission.



## create_project_permission

> crate::models::ProjectPermission create_project_permission(project_id, create_project_permission)
Create a project permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**create_project_permission** | Option<[**CreateProjectPermission**](CreateProjectPermission.md)> |  |  |

### Return type

[**crate::models::ProjectPermission**](ProjectPermission.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_permission

> crate::models::ProjectPermission get_project_permission(project_id, permission_id)
Retrieve a project permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**permission_id** | **String** |  | [required] |

### Return type

[**crate::models::ProjectPermission**](ProjectPermission.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_permissions

> crate::models::ProjectPermissionList get_project_permissions(project_id)
Retrieve a list of project permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**crate::models::ProjectPermissionList**](ProjectPermissionList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_permission

> crate::models::ProjectPermission update_project_permission(project_id, permission_id, if_match, project_permission)
Update a project permission.

Fields which can be updated: - uploadAllowed - downloadAllowed - roleProject - roleFlow - roleBase - roleBench

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**permission_id** | **String** |  | [required] |
**if_match** | Option<**String**> | Optional header parameter to enable conflict exposure. If the client provides this header, then it must contains the client's most recent value of the 'ETag' response header, and the server will respond with a 409 code if it detects a conflict. If the client does not provide this header, then the server will not do a conflict check, which means that as a client you can override the resource even when the server has a more recent version. |  |
**project_permission** | Option<[**ProjectPermission**](ProjectPermission.md)> |  |  |

### Return type

[**crate::models::ProjectPermission**](ProjectPermission.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

