# \ProjectPipelineApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cwl_pipeline**](ProjectPipelineApi.md#create_cwl_pipeline) | **POST** /api/projects/{projectId}/pipelines:createCwlPipeline | Create a CWL pipeline within a project.
[**create_nextflow_pipeline**](ProjectPipelineApi.md#create_nextflow_pipeline) | **POST** /api/projects/{projectId}/pipelines:createNextflowPipeline | Create a Nextflow pipeline within a project.
[**get_project_pipeline**](ProjectPipelineApi.md#get_project_pipeline) | **GET** /api/projects/{projectId}/pipelines/{pipelineId} | Retrieve a project pipeline.
[**get_project_pipeline_html_documentation**](ProjectPipelineApi.md#get_project_pipeline_html_documentation) | **GET** /api/projects/{projectId}/pipelines/{pipelineId}/documentation/HTML | Retrieve HTML documentation for a project pipeline.
[**get_project_pipeline_input_parameters**](ProjectPipelineApi.md#get_project_pipeline_input_parameters) | **GET** /api/projects/{projectId}/pipelines/{pipelineId}/inputParameters | Retrieve input parameters for a project pipeline.
[**get_project_pipeline_reference_sets**](ProjectPipelineApi.md#get_project_pipeline_reference_sets) | **GET** /api/projects/{projectId}/pipelines/{pipelineId}/referenceSets | Retrieve the reference sets of a project pipeline.
[**get_project_pipelines**](ProjectPipelineApi.md#get_project_pipelines) | **GET** /api/projects/{projectId}/pipelines | Retrieve a list of project pipelines.
[**link_pipeline_to_project**](ProjectPipelineApi.md#link_pipeline_to_project) | **POST** /api/projects/{projectId}/pipelines/{pipelineId} | Link a pipeline to a project.
[**release_pipeline**](ProjectPipelineApi.md#release_pipeline) | **POST** /api/projects/{projectId}/pipelines/{pipelineId}:release | Release a pipeline.
[**unlink_pipeline_from_project**](ProjectPipelineApi.md#unlink_pipeline_from_project) | **DELETE** /api/projects/{projectId}/pipelines/{pipelineId} | Unlink a pipeline from a project.



## create_cwl_pipeline

> crate::models::ProjectPipeline create_cwl_pipeline(project_id, code, description, workflow_cwl_file, parameters_xml_file, analysis_storage_id, tool_cwl_files, metadata_model_file, links, version_comment, categories, html_documentation)
Create a CWL pipeline within a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**code** | **String** | The code of the CWL pipeline | [required] |
**description** | **String** | The description of the CWL pipeline | [required] |
**workflow_cwl_file** | **std::path::PathBuf** | The CWL workflow file. | [required] |
**parameters_xml_file** | **std::path::PathBuf** |  | [required] |
**analysis_storage_id** | **uuid::Uuid** | The id of the storage to use for the pipeline. | [required] |
**tool_cwl_files** | Option<[**Vec<std::path::PathBuf>**](std::path::PathBuf.md)> |  |  |
**metadata_model_file** | Option<**std::path::PathBuf**> | The metadata model json file(contents can be retrieved from the controlplane). |  |
**links** | Option<[**crate::models::Links**](Links.md)> |  |  |
**version_comment** | Option<**String**> |  |  |
**categories** | Option<[**Vec<String>**](String.md)> |  |  |
**html_documentation** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectPipeline**](ProjectPipeline.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_nextflow_pipeline

> crate::models::ProjectPipeline create_nextflow_pipeline(project_id, code, description, main_nextflow_file, parameters_xml_file, analysis_storage_id, pipeline_language_version_id, nextflow_config_file, other_nextflow_files, metadata_model_file, links, version_comment, categories, html_documentation)
Create a Nextflow pipeline within a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**code** | **String** | The code of the pipeline | [required] |
**description** | **String** | The description of the pipeline | [required] |
**main_nextflow_file** | **std::path::PathBuf** | The main Nextflow file. | [required] |
**parameters_xml_file** | **std::path::PathBuf** |  | [required] |
**analysis_storage_id** | **uuid::Uuid** | The id of the storage to use for the pipeline. | [required] |
**pipeline_language_version_id** | Option<**uuid::Uuid**> | The id of the Nextflow version to use for the pipeline. |  |
**nextflow_config_file** | Option<**std::path::PathBuf**> | The Nextflow config file. |  |
**other_nextflow_files** | Option<[**Vec<std::path::PathBuf>**](std::path::PathBuf.md)> |  |  |
**metadata_model_file** | Option<**std::path::PathBuf**> | The metadata model json file(contents can be retrieved from the controlplane). |  |
**links** | Option<[**crate::models::Links**](Links.md)> |  |  |
**version_comment** | Option<**String**> |  |  |
**categories** | Option<[**Vec<String>**](String.md)> |  |  |
**html_documentation** | Option<**String**> |  |  |

### Return type

[**crate::models::ProjectPipeline**](ProjectPipeline.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_pipeline

> crate::models::ProjectPipeline get_project_pipeline(project_id, pipeline_id)
Retrieve a project pipeline.

Retrieves a project pipeline. This can be a pipeline from a linked bundle or an entitled, unlinked bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**pipeline_id** | **String** | The ID of the project pipeline to retrieve | [required] |

### Return type

[**crate::models::ProjectPipeline**](ProjectPipeline.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_pipeline_html_documentation

> crate::models::PipelineHtmlDocumentation get_project_pipeline_html_documentation(project_id, pipeline_id)
Retrieve HTML documentation for a project pipeline.

Retrieve HTML documentation for a project pipeline. This can be a pipeline from a linked bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**pipeline_id** | **String** | The ID of the project pipeline to retrieve HTML documentation from | [required] |

### Return type

[**crate::models::PipelineHtmlDocumentation**](PipelineHtmlDocumentation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_pipeline_input_parameters

> crate::models::InputParameterList get_project_pipeline_input_parameters(project_id, pipeline_id)
Retrieve input parameters for a project pipeline.

Retrieve input parameters for a project pipeline. This can be a pipeline from a linked bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**pipeline_id** | **String** | The ID of the project pipeline to retrieve input parameters for | [required] |

### Return type

[**crate::models::InputParameterList**](InputParameterList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_pipeline_reference_sets

> crate::models::ReferenceSetList get_project_pipeline_reference_sets(project_id, pipeline_id)
Retrieve the reference sets of a project pipeline.

Retrieve the reference sets of a project pipeline. This can be a pipeline from a linked bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**pipeline_id** | **String** | The ID of the pipeline to retrieve reference sets for | [required] |

### Return type

[**crate::models::ReferenceSetList**](ReferenceSetList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_pipelines

> crate::models::ProjectPipelineList get_project_pipelines(project_id)
Retrieve a list of project pipelines.

Lists all pipelines that are available to the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project to retrieve pipelines for | [required] |

### Return type

[**crate::models::ProjectPipelineList**](ProjectPipelineList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_pipeline_to_project

> link_pipeline_to_project(project_id, pipeline_id)
Link a pipeline to a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**pipeline_id** | **String** | The ID of the pipeline | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## release_pipeline

> release_pipeline(project_id, pipeline_id)
Release a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**pipeline_id** | **String** | The ID of the pipeline | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_pipeline_from_project

> unlink_pipeline_from_project(project_id, pipeline_id)
Unlink a pipeline from a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**pipeline_id** | **String** | The ID of the pipeline | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

