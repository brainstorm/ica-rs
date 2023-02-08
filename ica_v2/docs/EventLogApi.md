# \EventLogApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_event_logs**](EventLogApi.md#get_event_logs) | **GET** /api/eventLog | Retrieve a list of event logs.



## get_event_logs

> crate::models::EventLogList get_event_logs(code, code_filter_type, category, date_from, date_until, rows)
Retrieve a list of event logs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | Option<**String**> | Code |  |
**code_filter_type** | Option<**String**> | Code filter type |  |
**category** | Option<**String**> | Category |  |
**date_from** | Option<**String**> | Date from |  |
**date_until** | Option<**String**> | Date until |  |
**rows** | Option<**i32**> | Amount of rows to fetch. Maximum 250. Defaults to 250 |  |[default to 250]

### Return type

[**crate::models::EventLogList**](EventLogList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

