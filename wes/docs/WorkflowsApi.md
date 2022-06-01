# \WorkflowsApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_workflow**](WorkflowsApi.md#create_workflow) | **POST** /v1/workflows | Create a workflow
[**get_workflow**](WorkflowsApi.md#get_workflow) | **GET** /v1/workflows/{workflowId} | Get the details of a workflow
[**list_workflows**](WorkflowsApi.md#list_workflows) | **GET** /v1/workflows | Get a list of workflows
[**update_workflow**](WorkflowsApi.md#update_workflow) | **PATCH** /v1/workflows/{workflowId} | Update an existing workflow



## create_workflow

> crate::models::Workflow create_workflow(body)
Create a workflow

Creates a new workflow and version (if provided).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateWorkflowRequest**](CreateWorkflowRequest.md)> |  |  |

### Return type

[**crate::models::Workflow**](Workflow.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow

> crate::models::Workflow get_workflow(workflow_id)
Get the details of a workflow

Gets the details of a workflow with a given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_id** | **String** | ID of the workflow | [required] |

### Return type

[**crate::models::Workflow**](Workflow.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workflows

> crate::models::WorkflowList list_workflows(tenant_id, name, categories, include, page_size, page_token, sort)
Get a list of workflows

Gets a list of workflows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> | ID of the tenant |  |
**name** | Option<**String**> |  |  |
**categories** | Option<[**Vec<String>**](String.md)> |  |  |
**include** | Option<[**Vec<String>**](String.md)> | Comma-separated list of properties to include in the response |  |
**page_size** | Option<**i32**> | Number of items to include in a page. Value must be an integer between 1 and 1000. Only one of pageSize or pageToken can be specified. |  |[default to 10]
**page_token** | Option<**String**> | Page offset descriptor. Valid page tokens are included in the response. Only one of pageSize or pageToken can be specified. |  |
**sort** | Option<**String**> | Specifies the order to include list items as \"_{fieldName}_ [asc|desc]\". The second field is optional and specifies the sort direction (\"asc\" for ascending or \"desc\" for descending). |  |[default to timeCreated asc]

### Return type

[**crate::models::WorkflowList**](WorkflowList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workflow

> crate::models::Workflow update_workflow(workflow_id, body)
Update an existing workflow

Updates the workflow with a given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_id** | **String** | ID of the workflow | [required] |
**body** | Option<[**UpdateWorkflowRequest**](UpdateWorkflowRequest.md)> |  |  |

### Return type

[**crate::models::Workflow**](Workflow.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

