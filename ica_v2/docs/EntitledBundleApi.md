# \EntitledBundleApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_entitled_bundle**](EntitledBundleApi.md#get_entitled_bundle) | **GET** /api/entitledbundles/{entitledBundleId} | Retrieve an entitled bundle.
[**get_entitled_bundles**](EntitledBundleApi.md#get_entitled_bundles) | **GET** /api/entitledbundles | Retrieve a list of entitled bundles.



## get_entitled_bundle

> crate::models::Bundle get_entitled_bundle(entitled_bundle_id)
Retrieve an entitled bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entitled_bundle_id** | **uuid::Uuid** | The ID of the entitled bundle to retrieve | [required] |

### Return type

[**crate::models::Bundle**](Bundle.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_entitled_bundles

> crate::models::BundlePagedList get_entitled_bundles(search, user_tags, technical_tags, page_offset, page_token, page_size, sort)
Retrieve a list of entitled bundles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search |  |
**user_tags** | Option<**String**> | User tags to filter on |  |
**technical_tags** | Option<**String**> | Technical tags to filter on |  |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\"  The attributes for which sorting is supported: - name - shortDescription |  |

### Return type

[**crate::models::BundlePagedList**](BundlePagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

