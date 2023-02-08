# \DataApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_download_url_for_data_without_project_context**](DataApi.md#create_download_url_for_data_without_project_context) | **POST** /api/data/{dataUrn}:createDownloadUrl | Retrieve a download URL for this data.
[**create_inline_view_url_for_data_without_project_context**](DataApi.md#create_inline_view_url_for_data_without_project_context) | **POST** /api/data/{dataUrn}:createInlineViewUrl | Retrieve an URL for this data to use for inline view in a browser.
[**get_data**](DataApi.md#get_data) | **GET** /api/data/{dataUrn} | Retrieve a data.



## create_download_url_for_data_without_project_context

> crate::models::Download create_download_url_for_data_without_project_context(data_urn)
Retrieve a download URL for this data.

Can be used to download a file directly from the region where it is located, no connector is needed. Not applicable for Folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_urn** | **String** | The format is urn:ilmn:ica:region:\\<ID of the region\\>:data:\\<ID of the data\\>#\\<optional data path\\>. The path can be omitted, in that case the hashtag (#) must also be omitted. | [required] |

### Return type

[**crate::models::Download**](Download.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inline_view_url_for_data_without_project_context

> crate::models::InlineView create_inline_view_url_for_data_without_project_context(data_urn)
Retrieve an URL for this data to use for inline view in a browser.

Can be used to view a file directly from the region where it is located, no connector is needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_urn** | **String** | The format is urn:ilmn:ica:region:\\<ID of the region\\>:data:\\<ID of the data\\>#\\<optional data path\\>. The path can be omitted, in that case the hashtag (#) must also be omitted. | [required] |

### Return type

[**crate::models::InlineView**](InlineView.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data

> crate::models::Data get_data(data_urn)
Retrieve a data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_urn** | **String** | The format is urn:ilmn:ica:region:\\<ID of the region\\>:data:\\<ID of the data\\>#\\<optional data path\\>. The path can be omitted, in that case the hashtag (#) must also be omitted. | [required] |

### Return type

[**crate::models::Data**](Data.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

