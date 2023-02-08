# \WorkflowSignalsApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fail_signal**](WorkflowSignalsApi.md#fail_signal) | **PATCH** /v1/workflows/signals/{signalId}:fail | Fail a workflow signal
[**get_signal**](WorkflowSignalsApi.md#get_signal) | **GET** /v1/workflows/signals/{signalId} | Get the details of a workflow signal
[**list_signals**](WorkflowSignalsApi.md#list_signals) | **GET** /v1/workflows/signals | Get a list of workflow signals
[**succeed_signal**](WorkflowSignalsApi.md#succeed_signal) | **PATCH** /v1/workflows/signals/{signalId}:succeed | Succeed a workflow signal



## fail_signal

> crate::models::WorkflowSignal fail_signal(signal_id, body)
Fail a workflow signal

Responds to a pending workflow signal with a failure result.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signal_id** | **String** | ID of the workflow signal | [required] |
**body** | Option<[**FailWorkflowSignalRequest**](FailWorkflowSignalRequest.md)> |  |  |

### Return type

[**crate::models::WorkflowSignal**](WorkflowSignal.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_signal

> crate::models::WorkflowSignal get_signal(signal_id)
Get the details of a workflow signal

Gets the details of a workflow signal with a given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signal_id** | **String** | ID of the workflow signal | [required] |

### Return type

[**crate::models::WorkflowSignal**](WorkflowSignal.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_signals

> crate::models::WorkflowSignalList list_signals(tenant_id, include, page_size, page_token, sort)
Get a list of workflow signals

Gets a list of workflow signals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> | ID of the tenant |  |
**include** | Option<[**Vec<String>**](String.md)> | Comma-separated list of properties to include in the response |  |
**page_size** | Option<**i32**> | Number of items to include in a page. Value must be an integer between 1 and 1000. Only one of pageSize or pageToken can be specified. |  |[default to 10]
**page_token** | Option<**String**> | Page offset descriptor. Valid page tokens are included in the response. Only one of pageSize or pageToken can be specified. |  |
**sort** | Option<**String**> | Specifies the order to include list items as \"_{fieldName}_ [asc|desc]\". The second field is optional and specifies the sort direction (\"asc\" for ascending or \"desc\" for descending). |  |[default to timeCreated asc]

### Return type

[**crate::models::WorkflowSignalList**](WorkflowSignalList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## succeed_signal

> crate::models::WorkflowSignal succeed_signal(signal_id, body)
Succeed a workflow signal

Responds to a pending workflow signal with a successful result.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signal_id** | **String** | ID of the workflow signal | [required] |
**body** | Option<[**SucceedWorkflowSignalRequest**](SucceedWorkflowSignalRequest.md)> |  |  |

### Return type

[**crate::models::WorkflowSignal**](WorkflowSignal.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

