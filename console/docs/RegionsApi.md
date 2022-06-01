# \RegionsApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_regions**](RegionsApi.md#list_regions) | **GET** /v1/regions | Get a list of available regions



## list_regions

> Vec<crate::models::Region> list_regions(instrument_type, version)
Get a list of available regions

Get a list of available regions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument_type** | Option<**String**> | Instrument type |  |
**version** | Option<**String**> | Instrument version |  |

### Return type

[**Vec<crate::models::Region>**](Region.md)

### Authorization

[Basic](../README.md#Basic), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

