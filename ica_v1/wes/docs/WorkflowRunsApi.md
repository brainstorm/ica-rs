# \WorkflowRunsApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**abort_workflow_run**](WorkflowRunsApi.md#abort_workflow_run) | **PUT** /v1/workflows/runs/{runId}:abort | Abort a workflow run
[**get_workflow_run**](WorkflowRunsApi.md#get_workflow_run) | **GET** /v1/workflows/runs/{runId} | Get the details of a workflow run
[**list_workflow_run_history**](WorkflowRunsApi.md#list_workflow_run_history) | **GET** /v1/workflows/runs/{runId}/history | Get a list of workflow run history events
[**list_workflow_runs**](WorkflowRunsApi.md#list_workflow_runs) | **GET** /v1/workflows/runs | Get a list of workflow runs



## abort_workflow_run

> crate::models::WorkflowRun abort_workflow_run(run_id, include, body)
Abort a workflow run

Aborts the workflow run with a given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** | ID of the workflow run | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Comma-separated list of properties to include in the response |  |
**body** | Option<[**AbortWorkflowRunRequest**](AbortWorkflowRunRequest.md)> |  |  |

### Return type

[**crate::models::WorkflowRun**](WorkflowRun.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_run

> crate::models::WorkflowRun get_workflow_run(run_id, include)
Get the details of a workflow run

Gets the details of a workflow run with a given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** | ID of the workflow run | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Comma-separated list of properties to include in the response |  |

### Return type

[**crate::models::WorkflowRun**](WorkflowRun.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workflow_run_history

> crate::models::WorkflowRunHistoryEventList list_workflow_run_history(run_id, sort, include, page_size, page_token)
Get a list of workflow run history events

Gets a list of history events for a workflow run with a given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** | ID of the workflow run | [required] |
**sort** | Option<**String**> |  |  |[default to eventId asc]
**include** | Option<[**Vec<String>**](String.md)> | Comma-separated list of properties to include in the response |  |
**page_size** | Option<**i32**> | Number of items to include in a page. Value must be an integer between 1 and 1000. Only one of pageSize or pageToken can be specified. |  |[default to 10]
**page_token** | Option<**String**> | Page offset descriptor. Valid page tokens are included in the response. Only one of pageSize or pageToken can be specified. |  |

### Return type

[**crate::models::WorkflowRunHistoryEventList**](WorkflowRunHistoryEventList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workflow_runs

> crate::models::WorkflowRunList list_workflow_runs(status, tenant_id, name, include, page_size, page_token, sort)
Get a list of workflow runs

Gets a list of workflow runs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**Vec<String>**](String.md)> |  |  |
**tenant_id** | Option<**String**> | ID of the tenant |  |
**name** | Option<**String**> |  |  |
**include** | Option<[**Vec<String>**](String.md)> | Comma-separated list of properties to include in the response |  |
**page_size** | Option<**i32**> | Number of items to include in a page. Value must be an integer between 1 and 1000. Only one of pageSize or pageToken can be specified. |  |[default to 10]
**page_token** | Option<**String**> | Page offset descriptor. Valid page tokens are included in the response. Only one of pageSize or pageToken can be specified. |  |
**sort** | Option<**String**> | Specifies the order to include list items as \"_{fieldName}_ [asc|desc]\". The second field is optional and specifies the sort direction (\"asc\" for ascending or \"desc\" for descending). |  |[default to timeCreated asc]

### Return type

[**crate::models::WorkflowRunList**](WorkflowRunList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

