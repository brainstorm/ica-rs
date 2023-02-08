# \BundleSampleApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bundle_samples**](BundleSampleApi.md#get_bundle_samples) | **GET** /api/bundles/{bundleId}/samples | Retrieve a list of bundle samples.
[**link_sample_to_bundle**](BundleSampleApi.md#link_sample_to_bundle) | **POST** /api/bundles/{bundleId}/samples/{sampleId} | Link a sample to a bundle.
[**unlink_sample_from_bundle**](BundleSampleApi.md#unlink_sample_from_bundle) | **DELETE** /api/bundles/{bundleId}/samples/{sampleId} | Unlink a sample from a bundle.



## get_bundle_samples

> crate::models::BundleSamplePagedList get_bundle_samples(bundle_id, search, user_tags, technical_tags, page_offset, page_token, page_size, sort)
Retrieve a list of bundle samples.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** | The ID of the bundle to get bundle samples from | [required] |
**search** | Option<**String**> | To search through multiple fields of data. |  |
**user_tags** | Option<**String**> | The user tags to filter on. |  |
**technical_tags** | Option<**String**> | The technical tags to filter on. |  |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\" The attributes for which sorting is supported: - timeCreated - timeModified - name - description - metadataValid - status |  |

### Return type

[**crate::models::BundleSamplePagedList**](BundleSamplePagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_sample_to_bundle

> link_sample_to_bundle(bundle_id, sample_id)
Link a sample to a bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** |  | [required] |
**sample_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_sample_from_bundle

> unlink_sample_from_bundle(bundle_id, sample_id)
Unlink a sample from a bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** |  | [required] |
**sample_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

