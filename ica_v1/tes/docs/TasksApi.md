# \TasksApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task**](TasksApi.md#create_task) | **POST** /v1/tasks | Create a Task
[**get_task**](TasksApi.md#get_task) | **GET** /v1/tasks/{taskId} | Get the details of a Task
[**list_tasks**](TasksApi.md#list_tasks) | **GET** /v1/tasks | Get a list of tasks
[**update_task**](TasksApi.md#update_task) | **PATCH** /v1/tasks/{taskId} | Update an existing task.



## create_task

> crate::models::Task create_task(body)
Create a Task

Creates a task. Returns the ID associated with the new task. Also returns the task version ID associated with the new task, if provided. Substitutions can be defined in the following format: \"{{string}}\", and specified at launch time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateTaskRequest**](CreateTaskRequest.md)> |  |  |

### Return type

[**crate::models::Task**](Task.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task

> crate::models::TaskSummary get_task(task_id)
Get the details of a Task

Gets the details of a Task for a given task ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |

### Return type

[**crate::models::TaskSummary**](TaskSummary.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tasks

> crate::models::TaskSummaryPagedItems list_tasks(names, acls, page_size, sort, page_token)
Get a list of tasks

Gets a list of tasks accessible by the current tenant ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**names** | Option<**String**> | Name: Optional parameter to filter the returned list. Case-Sensitive |  |
**acls** | Option<**String**> | Name: Optional parameter to filter the returned list. Case-Sensitive |  |
**page_size** | Option<**i32**> | Optional parameter to define the page size returned. Valid inputs range from 1-1000. |  |[default to 10]
**sort** | Option<**String**> | Sort: Optional parameter to set the sort of the returned list. Valid fields include: name, timeCreated, timeModified.  The sort can be specified as asc or desc. (Default: asc.) |  |[default to timeCreated asc]
**page_token** | Option<**String**> | pageToken: Optional parameter for navigation after initial listing. Valid values include firstPageToken,  nextPageToken, and previousPageToken (provided in the list response) |  |

### Return type

[**crate::models::TaskSummaryPagedItems**](TaskSummaryPagedItems.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task

> crate::models::Task update_task(task_id, body)
Update an existing task.

Updates the task with a given ID. The task's name, description can be updated. The task's name must remain unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |
**body** | Option<[**UpdateTaskRequest**](UpdateTaskRequest.md)> | Details of the task to be updated. |  |

### Return type

[**crate::models::Task**](Task.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

