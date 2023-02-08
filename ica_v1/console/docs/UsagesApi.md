# \UsagesApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_usage**](UsagesApi.md#get_usage) | **GET** /v1/usages | Get current tenant's usage detail by period.  Default returns current period usage data. 
[**get_usage_details**](UsagesApi.md#get_usage_details) | **GET** /v1/usages/details | Get current tenant's usage detail by period.  Default returns current period usage data. 
[**get_usage_periods**](UsagesApi.md#get_usage_periods) | **GET** /v1/usages/periods | Get periods detail info 



## get_usage

> crate::models::UsageResponse get_usage(periods)
Get current tenant's usage detail by period.  Default returns current period usage data. 

This endpoint provides the ability for the user to get the aggregated usage data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**periods** | Option<**i32**> |  |  |

### Return type

[**crate::models::UsageResponse**](UsageResponse.md)

### Authorization

[Basic](../README.md#Basic), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_details

> crate::models::UsageResponse get_usage_details(period_id)
Get current tenant's usage detail by period.  Default returns current period usage data. 

This endpoint provides the billing details for specified period id. Summarize each compute usage and daily gds usage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**period_id** | Option<**i64**> |  |  |

### Return type

[**crate::models::UsageResponse**](UsageResponse.md)

### Authorization

[Basic](../README.md#Basic), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_periods

> crate::models::UsageResponse get_usage_periods(limit)
Get periods detail info 

This endpoint provides the periods details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 26]

### Return type

[**crate::models::UsageResponse**](UsageResponse.md)

### Authorization

[Basic](../README.md#Basic), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

