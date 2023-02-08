# \DataFormatApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_data_formats**](DataFormatApi.md#get_data_formats) | **GET** /api/dataFormats | Retrieve a list of data formats.



## get_data_formats

> crate::models::DataFormatPagedList get_data_formats(page_offset, page_token, page_size, sort)
Retrieve a list of data formats.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\" |  |

### Return type

[**crate::models::DataFormatPagedList**](DataFormatPagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

