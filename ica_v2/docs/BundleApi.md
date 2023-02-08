# \BundleApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bundle**](BundleApi.md#create_bundle) | **POST** /api/bundles | Create a new bundle
[**deprecate_bundle**](BundleApi.md#deprecate_bundle) | **POST** /api/bundles/{bundleId}:deprecate | deprecate a bundle
[**get_bundle**](BundleApi.md#get_bundle) | **GET** /api/bundles/{bundleId} | Retrieve a bundle.
[**get_bundle_terms_of_use**](BundleApi.md#get_bundle_terms_of_use) | **GET** /api/bundles/{bundleId}/termsOfUse | Retrieve the last version of terms of use for a bundle.
[**get_bundles**](BundleApi.md#get_bundles) | **GET** /api/bundles | Retrieve a list of bundles.
[**insert_bundle_terms_of_use**](BundleApi.md#insert_bundle_terms_of_use) | **POST** /api/bundles/{bundleId}/termsOfUse:new | Insert a new version of the terms of use for a bundle
[**release_bundle**](BundleApi.md#release_bundle) | **POST** /api/bundles/{bundleId}:release | release a bundle



## create_bundle

> crate::models::Bundle create_bundle(create_bundle)
Create a new bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_bundle** | Option<[**CreateBundle**](CreateBundle.md)> |  |  |

### Return type

[**crate::models::Bundle**](Bundle.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deprecate_bundle

> deprecate_bundle(bundle_id)
deprecate a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** | The ID of the bundle to deprecate. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bundle

> crate::models::Bundle get_bundle(bundle_id)
Retrieve a bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** | The ID of the bundle to retrieve | [required] |

### Return type

[**crate::models::Bundle**](Bundle.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bundle_terms_of_use

> crate::models::TermsOfUse get_bundle_terms_of_use(bundle_id)
Retrieve the last version of terms of use for a bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** | The ID of the bundle of the terms of use to retrieve | [required] |

### Return type

[**crate::models::TermsOfUse**](TermsOfUse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bundles

> crate::models::BundlePagedList get_bundles(search, user_tags, technical_tags, page_offset, page_token, page_size, sort)
Retrieve a list of bundles.

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


## insert_bundle_terms_of_use

> crate::models::TermsOfUse insert_bundle_terms_of_use(bundle_id, create_terms_of_use)
Insert a new version of the terms of use for a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **uuid::Uuid** | The ID of the bundle to update | [required] |
**create_terms_of_use** | Option<[**CreateTermsOfUse**](CreateTermsOfUse.md)> |  |  |

### Return type

[**crate::models::TermsOfUse**](TermsOfUse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/x-www-form-urlencoded, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## release_bundle

> release_bundle(bundle_id)
release a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** | The ID of the bundle to release | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

