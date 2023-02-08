# \ProjectCustomEventsApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_event**](ProjectCustomEventsApi.md#create_custom_event) | **POST** /api/projects/{projectId}/customEvents | Create a new custom event.



## create_custom_event

> create_custom_event(project_id, create_custom_event)
Create a new custom event.

Warning: this endpoint allows to create custom events with a code larger than 20 characters (max 50), but the endpoint for creating a custom notification subscription (POST /api/projects/{projectId}/customNotificationSubscriptions) only accepts event codes up to 20 characters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**create_custom_event** | [**CreateCustomEvent**](CreateCustomEvent.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

