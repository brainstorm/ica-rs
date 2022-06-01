# \WorkflowVersionsApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_workflow_version**](WorkflowVersionsApi.md#create_workflow_version) | **POST** /v1/workflows/{workflowId}/versions | Create a new workflow version
[**get_workflow_version**](WorkflowVersionsApi.md#get_workflow_version) | **GET** /v1/workflows/{workflowId}/versions/{versionName} | Get the details of a workflow version
[**launch_workflow_version**](WorkflowVersionsApi.md#launch_workflow_version) | **POST** /v1/workflows/{workflowId}/versions/{versionName}:launch | Launch a workflow version
[**list_all_workflow_versions**](WorkflowVersionsApi.md#list_all_workflow_versions) | **GET** /v1/workflows/versions | Get a list of all workflow versions
[**list_workflow_versions**](WorkflowVersionsApi.md#list_workflow_versions) | **GET** /v1/workflows/{workflowId}/versions | Get a list of workflow versions
[**update_workflow_version**](WorkflowVersionsApi.md#update_workflow_version) | **PATCH** /v1/workflows/{workflowId}/versions/{versionName} | Update an existing workflow version



## create_workflow_version

> crate::models::WorkflowVersion create_workflow_version(workflow_id, body)
Create a new workflow version

Creates a new workflow version with a given workflow ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_id** | **String** | ID of the workflow | [required] |
**body** | Option<[**CreateWorkflowVersionRequest**](CreateWorkflowVersionRequest.md)> |  |  |

### Return type

[**crate::models::WorkflowVersion**](WorkflowVersion.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_version

> crate::models::WorkflowVersion get_workflow_version(workflow_id, version_name)
Get the details of a workflow version

Gets the details for a workflow version with a given workflow ID and version name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_id** | **String** | ID of the workflow | [required] |
**version_name** | **String** | Name of the workflow version | [required] |

### Return type

[**crate::models::WorkflowVersion**](WorkflowVersion.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## launch_workflow_version

> crate::models::WorkflowRun launch_workflow_version(workflow_id, version_name, include, body)
Launch a workflow version

Launches a workflow version with a given workflow ID and version name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_id** | **String** | ID of the workflow | [required] |
**version_name** | **String** | Name of the workflow version | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Comma-separated list of properties to include in the response |  |
**body** | Option<[**LaunchWorkflowVersionRequest**](LaunchWorkflowVersionRequest.md)> |  |  |

### Return type

[**crate::models::WorkflowRun**](WorkflowRun.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_all_workflow_versions

> crate::models::WorkflowVersionList list_all_workflow_versions(tenant_id, include, page_size, page_token, sort)
Get a list of all workflow versions

Gets a list of workflow versions across all workflows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> | ID of the tenant |  |
**include** | Option<[**Vec<String>**](String.md)> | Comma-separated list of properties to include in the response |  |
**page_size** | Option<**i32**> | Number of items to include in a page. Value must be an integer between 1 and 1000. Only one of pageSize or pageToken can be specified. |  |[default to 10]
**page_token** | Option<**String**> | Page offset descriptor. Valid page tokens are included in the response. Only one of pageSize or pageToken can be specified. |  |
**sort** | Option<**String**> | Specifies the order to include list items as \"_{fieldName}_ [asc|desc]\". The second field is optional and specifies the sort direction (\"asc\" for ascending or \"desc\" for descending). |  |[default to timeCreated asc]

### Return type

[**crate::models::WorkflowVersionList**](WorkflowVersionList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workflow_versions

> crate::models::WorkflowVersionList list_workflow_versions(workflow_id, include, page_size, page_token, sort)
Get a list of workflow versions

Gets a list of workflow versions with a given workflow ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_id** | **String** | ID of the workflow | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Comma-separated list of properties to include in the response |  |
**page_size** | Option<**i32**> | Number of items to include in a page. Value must be an integer between 1 and 1000. Only one of pageSize or pageToken can be specified. |  |[default to 10]
**page_token** | Option<**String**> | Page offset descriptor. Valid page tokens are included in the response. Only one of pageSize or pageToken can be specified. |  |
**sort** | Option<**String**> | Specifies the order to include list items as \"_{fieldName}_ [asc|desc]\". The second field is optional and specifies the sort direction (\"asc\" for ascending or \"desc\" for descending). |  |[default to timeCreated asc]

### Return type

[**crate::models::WorkflowVersionList**](WorkflowVersionList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workflow_version

> crate::models::WorkflowVersion update_workflow_version(workflow_id, version_name, body)
Update an existing workflow version

Updates an existing workflow version. Note: The Version, Definition, and Status cannot be changed simultaneously. Only one of these can be changed per API call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_id** | **String** | ID of the workflow | [required] |
**version_name** | **String** | Name of the workflow version | [required] |
**body** | Option<[**UpdateWorkflowVersionRequest**](UpdateWorkflowVersionRequest.md)> |  |  |

### Return type

[**crate::models::WorkflowVersion**](WorkflowVersion.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

