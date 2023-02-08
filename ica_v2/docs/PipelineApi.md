# \PipelineApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_pipeline**](PipelineApi.md#get_pipeline) | **GET** /api/pipelines/{pipelineId} | Retrieve a pipeline.
[**get_pipeline_html_documentation**](PipelineApi.md#get_pipeline_html_documentation) | **GET** /api/pipelines/{pipelineId}/documentation/HTML | Retrieve HTML documentation for a project pipeline.
[**get_pipeline_input_parameters**](PipelineApi.md#get_pipeline_input_parameters) | **GET** /api/pipelines/{pipelineId}/inputParameters | Retrieve input parameters for a pipeline.
[**get_pipeline_reference_sets**](PipelineApi.md#get_pipeline_reference_sets) | **GET** /api/pipelines/{pipelineId}/referenceSets | Retrieve the reference sets of a pipeline.
[**get_pipelines**](PipelineApi.md#get_pipelines) | **GET** /api/pipelines | Retrieve a list of pipelines.



## get_pipeline

> crate::models::Pipeline get_pipeline(pipeline_id)
Retrieve a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** | The ID of the pipeline to retrieve | [required] |

### Return type

[**crate::models::Pipeline**](Pipeline.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_html_documentation

> crate::models::PipelineHtmlDocumentation get_pipeline_html_documentation(pipeline_id)
Retrieve HTML documentation for a project pipeline.

Retrieve HTML documentation for a project pipeline. This can be a pipeline from a linked bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** | The ID of the project pipeline to retrieve HTML documentation from | [required] |

### Return type

[**crate::models::PipelineHtmlDocumentation**](PipelineHtmlDocumentation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_input_parameters

> crate::models::InputParameterList get_pipeline_input_parameters(pipeline_id)
Retrieve input parameters for a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** | The ID of the pipeline to retrieve input parameters for | [required] |

### Return type

[**crate::models::InputParameterList**](InputParameterList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_reference_sets

> crate::models::ReferenceSetList get_pipeline_reference_sets(pipeline_id)
Retrieve the reference sets of a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** | The ID of the pipeline to retrieve reference sets for | [required] |

### Return type

[**crate::models::ReferenceSetList**](ReferenceSetList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipelines

> crate::models::PipelineList get_pipelines()
Retrieve a list of pipelines.

Only lists pipelines that are owned by the user/tenant (not those to which a user is entitled).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PipelineList**](PipelineList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

