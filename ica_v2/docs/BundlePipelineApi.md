# \BundlePipelineApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bundle_pipelines**](BundlePipelineApi.md#get_bundle_pipelines) | **GET** /api/bundles/{bundleId}/pipelines | Retrieve a list of bundle pipelines.
[**link_pipeline_to_bundle**](BundlePipelineApi.md#link_pipeline_to_bundle) | **POST** /api/bundles/{bundleId}/pipelines/{pipelineId} | Link a pipeline to a bundle.
[**unlink_pipeline_from_bundle**](BundlePipelineApi.md#unlink_pipeline_from_bundle) | **DELETE** /api/bundles/{bundleId}/pipelines/{pipelineId} | Unlink a pipeline from a bundle.



## get_bundle_pipelines

> crate::models::BundlePipelineList get_bundle_pipelines(bundle_id)
Retrieve a list of bundle pipelines.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** | The ID of the bundle to retrieve pipelines for | [required] |

### Return type

[**crate::models::BundlePipelineList**](BundlePipelineList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_pipeline_to_bundle

> link_pipeline_to_bundle(bundle_id, pipeline_id)
Link a pipeline to a bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** | The ID of the bundle | [required] |
**pipeline_id** | **String** | The ID of the pipeline | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_pipeline_from_bundle

> unlink_pipeline_from_bundle(bundle_id, pipeline_id)
Unlink a pipeline from a bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** | The ID of the bundle | [required] |
**pipeline_id** | **String** | The ID of the pipeline | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

