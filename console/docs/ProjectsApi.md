# \ProjectsApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_projects**](ProjectsApi.md#list_projects) | **GET** /v1/projects | Get a list of available projects. Requires user authorization Bearer token.



## list_projects

> crate::models::ProjectPagedItems list_projects(page_token)
Get a list of available projects. Requires user authorization Bearer token.

Get a list of available projects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_token** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectPagedItems**](ProjectPagedItems.md)

### Authorization

[Basic](../README.md#Basic), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

