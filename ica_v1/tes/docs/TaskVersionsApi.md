# \TaskVersionsApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task_version**](TaskVersionsApi.md#create_task_version) | **POST** /v1/tasks/{taskId}/versions | Create a task version
[**get_task_version**](TaskVersionsApi.md#get_task_version) | **GET** /v1/tasks/{taskId}/versions/{versionId} | Get the details of a task version
[**launch_task_run**](TaskVersionsApi.md#launch_task_run) | **POST** /v1/tasks/{taskId}/versions/{versionId}:launch | Launch a task version
[**list_task_versions**](TaskVersionsApi.md#list_task_versions) | **GET** /v1/tasks/{taskId}/versions | Get a list of versions
[**update_task_version**](TaskVersionsApi.md#update_task_version) | **PATCH** /v1/tasks/{taskId}/versions/{versionId} | Update task version properties



## create_task_version

> crate::models::TaskVersion create_task_version(task_id, body)
Create a task version

Creates a new task version within an existing task. Returns the ID associated with the new task version. Substitutions can be defined in the following format: \"{{string}}\", and specified at launch time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |
**body** | Option<[**CreateTaskVersionRequest**](CreateTaskVersionRequest.md)> |  |  |

### Return type

[**crate::models::TaskVersion**](TaskVersion.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task_version

> crate::models::TaskVersion get_task_version(task_id, version_id)
Get the details of a task version

Gets details of a task version for a given task version ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

[**crate::models::TaskVersion**](TaskVersion.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## launch_task_run

> crate::models::TaskRun launch_task_run(task_id, version_id, body)
Launch a task version

Launches a task version for a given task version ID. Returns the ID associated with the new task run. Substitutions defined in the task version must be specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |
**body** | Option<[**LaunchTaskRequest**](LaunchTaskRequest.md)> |  |  |

### Return type

[**crate::models::TaskRun**](TaskRun.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_task_versions

> crate::models::TaskVersionSummaryPagedItems list_task_versions(task_id, sort, versions, ids, acls, page_size, page_token)
Get a list of versions

Gets a list of task versions within the given task accessible by the current tenant ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |
**sort** | Option<**String**> | Sort: Optional parameter to set the sort of the returned list. Valid fields include: name, version, timeCreated, timeModified.  The sort can be specified as asc or desc. (Default: asc.) |  |
**versions** | Option<**String**> |  |  |
**ids** | Option<**String**> |  |  |
**acls** | Option<**String**> |  |  |
**page_size** | Option<**i32**> | Optional parameter to define the page size returned. Valid inputs range from 1-1000. |  |[default to 10]
**page_token** | Option<**String**> | pageToken: Optional parameter for navigation after initial listing. Valid values include firstPageToken,  nextPageToken, and previousPageToken (provided in the list response) |  |

### Return type

[**crate::models::TaskVersionSummaryPagedItems**](TaskVersionSummaryPagedItems.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task_version

> crate::models::TaskVersion update_task_version(task_id, version_id, body)
Update task version properties

Update details of a task version for a given task version ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |
**body** | Option<[**UpdateTaskVersionRequest**](UpdateTaskVersionRequest.md)> |  |  |

### Return type

[**crate::models::TaskVersion**](TaskVersion.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

