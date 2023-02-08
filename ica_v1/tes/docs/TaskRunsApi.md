# \TaskRunsApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**abort_task_run**](TaskRunsApi.md#abort_task_run) | **PUT** /v1/tasks/runs/{runId}:abort | Abort a task run
[**create_task_run**](TaskRunsApi.md#create_task_run) | **POST** /v1/tasks/runs | Create and launch a task run
[**get_task_run**](TaskRunsApi.md#get_task_run) | **GET** /v1/tasks/runs/{runId} | Get the status of a task run
[**heartbeat_task_run**](TaskRunsApi.md#heartbeat_task_run) | **POST** /v1/tasks/runs/{runId}:heartbeat | Heartbeat for a task run
[**list_task_runs**](TaskRunsApi.md#list_task_runs) | **GET** /v1/tasks/runs | Get a list of task runs



## abort_task_run

> crate::models::TaskRunSummary abort_task_run(run_id)
Abort a task run

Aborts a task run for a given task run ID. The task run is required to have a status of \"Pending\" or \"Running\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |

### Return type

[**crate::models::TaskRunSummary**](TaskRunSummary.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_task_run

> crate::models::TaskRun create_task_run(body)
Create and launch a task run

Creates and launches a task run. Returns the ID and status associated with the new task run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateTaskRunRequest**](CreateTaskRunRequest.md)> |  |  |

### Return type

[**crate::models::TaskRun**](TaskRun.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task_run

> crate::models::TaskRun get_task_run(run_id)
Get the status of a task run

Gets the status of a task run for a given task run ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |

### Return type

[**crate::models::TaskRun**](TaskRun.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## heartbeat_task_run

> crate::models::TaskRunHeartbeat heartbeat_task_run(run_id, body)
Heartbeat for a task run

Heartbeat a task run for a given task run ID. This notifies TES that a task run is executing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |
**body** | Option<[**HeartbeatTaskRunRequest**](HeartbeatTaskRunRequest.md)> |  |  |

### Return type

[**crate::models::TaskRunHeartbeat**](TaskRunHeartbeat.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_task_runs

> crate::models::TaskRunSummaryPagedItems list_task_runs(sort, names, status, versions, acls, page_size, page_token)
Get a list of task runs

Get a list of task runs accessible by the current tenant ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Sort: Optional parameter to set the sort of the returned list. Valid fields include: name, status, timeCreated, timeModified.  The sort can be specified as asc or desc. (Default: asc.) |  |
**names** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**versions** | Option<**String**> |  |  |
**acls** | Option<**String**> |  |  |
**page_size** | Option<**i32**> | Optional parameter to define the page size returned. Valid inputs range from 1-1000. |  |[default to 10]
**page_token** | Option<**String**> | pageToken: Optional parameter for navigation after initial listing. Valid values include firstPageToken,  nextPageToken, and previousPageToken (provided in the list response) |  |

### Return type

[**crate::models::TaskRunSummaryPagedItems**](TaskRunSummaryPagedItems.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

