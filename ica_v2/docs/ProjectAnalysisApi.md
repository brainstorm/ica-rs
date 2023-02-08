# \ProjectAnalysisApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**abort_analysis**](ProjectAnalysisApi.md#abort_analysis) | **POST** /api/projects/{projectId}/analyses/{analysisId}:abort | Abort an analysis.
[**create_cwl_analysis**](ProjectAnalysisApi.md#create_cwl_analysis) | **POST** /api/projects/{projectId}/analysis:cwl | Create and start an analysis for a CWL pipeline.
[**create_nextflow_analysis**](ProjectAnalysisApi.md#create_nextflow_analysis) | **POST** /api/projects/{projectId}/analysis:nextflow | Create and start an analysis for a Nextflow pipeline.
[**get_analyses**](ProjectAnalysisApi.md#get_analyses) | **GET** /api/projects/{projectId}/analyses | Retrieve the list of analyses.
[**get_analysis**](ProjectAnalysisApi.md#get_analysis) | **GET** /api/projects/{projectId}/analyses/{analysisId} | Retrieve an analysis.
[**get_analysis_configurations**](ProjectAnalysisApi.md#get_analysis_configurations) | **GET** /api/projects/{projectId}/analyses/{analysisId}/configurations | Retrieve the configurations of an analysis.
[**get_analysis_inputs**](ProjectAnalysisApi.md#get_analysis_inputs) | **GET** /api/projects/{projectId}/analyses/{analysisId}/inputs | Retrieve the inputs of an analysis.
[**get_analysis_outputs**](ProjectAnalysisApi.md#get_analysis_outputs) | **GET** /api/projects/{projectId}/analyses/{analysisId}/outputs | Retrieve the outputs of an analysis. The list might be incomplete if a folder contains too many output files, but all the data can always be retrieved through the GET data endpoint.
[**get_analysis_steps**](ProjectAnalysisApi.md#get_analysis_steps) | **GET** /api/projects/{projectId}/analyses/{analysisId}/steps | Retrieve the individual steps of an analysis.
[**get_cwl_input_json**](ProjectAnalysisApi.md#get_cwl_input_json) | **GET** /api/projects/{projectId}/analyses/{analysisId}/cwl/inputJson | Retrieve the input json of a CWL analysis.
[**get_cwl_output_json**](ProjectAnalysisApi.md#get_cwl_output_json) | **GET** /api/projects/{projectId}/analyses/{analysisId}/cwl/outputJson | Retrieve the output json of a CWL analysis.
[**get_raw_analysis_output**](ProjectAnalysisApi.md#get_raw_analysis_output) | **GET** /api/projects/{projectId}/analyses/{analysisId}/rawOutput | Retrieve the raw output of an analysis.
[**update_analysis**](ProjectAnalysisApi.md#update_analysis) | **PUT** /api/projects/{projectId}/analyses/{analysisId} | Update an analysis.



## abort_analysis

> abort_analysis(project_id, analysis_id)
Abort an analysis.

Endpoint for aborting an analysis. The status of the analysis is not updated immediately, only when the abortion of the analysis has actually started.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**analysis_id** | **String** | The ID of the analysis to abort | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cwl_analysis

> crate::models::Analysis create_cwl_analysis(project_id, idempotency_key, create_cwl_analysis)
Create and start an analysis for a CWL pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**idempotency_key** | Option<**String**> | The Idempotency-Key header can be used to prevent duplicate requests and support retries. It is implemented according to <a href=\"https://tools.ietf.org/id/draft-idempotency-header-01.html\">the IETF spec</a> and is allowed to be max 255 characters long. If the header is supplied, the response of the request will be saved for 7 days for the specific API endpoint, header value and user reference. When the same user makes a new request within 7 days to the same API endpoint with the same Idempotency-Key header value, following use cases can occur:<br /><ul><li>the request body is the same as the previous request and an answer is stored => the stored response is returned without executing the request again.</li><li>the request body is the same as the previous request and no answer is stored because the previous request has not finished => 409 error response.</li><li>the request body is not the same as the previous request => 422 error response.</li></ul>This means that each time when executing an API request using the Idempotency-Key header, the request has to contain a new value that hasn't been used in the past 7 days for that specific API and by the specific user. |  |
**create_cwl_analysis** | Option<[**CreateCwlAnalysis**](CreateCwlAnalysis.md)> |  |  |

### Return type

[**crate::models::Analysis**](Analysis.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_nextflow_analysis

> crate::models::Analysis create_nextflow_analysis(project_id, idempotency_key, create_nextflow_analysis)
Create and start an analysis for a Nextflow pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**idempotency_key** | Option<**String**> | The Idempotency-Key header can be used to prevent duplicate requests and support retries. It is implemented according to <a href=\"https://tools.ietf.org/id/draft-idempotency-header-01.html\">the IETF spec</a> and is allowed to be max 255 characters long. If the header is supplied, the response of the request will be saved for 7 days for the specific API endpoint, header value and user reference. When the same user makes a new request within 7 days to the same API endpoint with the same Idempotency-Key header value, following use cases can occur:<br /><ul><li>the request body is the same as the previous request and an answer is stored => the stored response is returned without executing the request again.</li><li>the request body is the same as the previous request and no answer is stored because the previous request has not finished => 409 error response.</li><li>the request body is not the same as the previous request => 422 error response.</li></ul>This means that each time when executing an API request using the Idempotency-Key header, the request has to contain a new value that hasn't been used in the past 7 days for that specific API and by the specific user. |  |
**create_nextflow_analysis** | Option<[**CreateNextflowAnalysis**](CreateNextflowAnalysis.md)> |  |  |

### Return type

[**crate::models::Analysis**](Analysis.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analyses

> crate::models::AnalysisPagedList get_analyses(project_id, reference, userreference, status, usertag, technicaltag, referencetag, page_offset, page_token, page_size, sort)
Retrieve the list of analyses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**reference** | Option<**String**> | The reference to filter on. |  |
**userreference** | Option<**String**> | The user-reference to filter on. |  |
**status** | Option<**String**> | The status to filter on. |  |
**usertag** | Option<**String**> | The user-tags to filter on. |  |
**technicaltag** | Option<**String**> | The technical-tags to filter on. |  |
**referencetag** | Option<**String**> | The reference-data-tags to filter on. |  |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\"  The attributes for which sorting is supported: - reference - userReference - pipeline - status - startDate - endDate - summary  |  |

### Return type

[**crate::models::AnalysisPagedList**](AnalysisPagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analysis

> crate::models::Analysis get_analysis(project_id, analysis_id)
Retrieve an analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**analysis_id** | **String** | The ID of the analysis to retrieve | [required] |

### Return type

[**crate::models::Analysis**](Analysis.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analysis_configurations

> crate::models::ExecutionConfigurationList get_analysis_configurations(project_id, analysis_id)
Retrieve the configurations of an analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**analysis_id** | **String** | The ID of the analysis to retrieve the configuration for | [required] |

### Return type

[**crate::models::ExecutionConfigurationList**](ExecutionConfigurationList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analysis_inputs

> crate::models::AnalysisInputList get_analysis_inputs(project_id, analysis_id)
Retrieve the inputs of an analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**analysis_id** | **String** | The ID of the analysis to retrieve the inputs for | [required] |

### Return type

[**crate::models::AnalysisInputList**](AnalysisInputList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analysis_outputs

> crate::models::AnalysisOutputList get_analysis_outputs(project_id, analysis_id)
Retrieve the outputs of an analysis. The list might be incomplete if a folder contains too many output files, but all the data can always be retrieved through the GET data endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**analysis_id** | **String** | The ID of the analysis to retrieve the outputs for | [required] |

### Return type

[**crate::models::AnalysisOutputList**](AnalysisOutputList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analysis_steps

> crate::models::AnalysisStepList get_analysis_steps(project_id, analysis_id)
Retrieve the individual steps of an analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**analysis_id** | **String** | The ID of the analysis to retrieve the individual steps for | [required] |

### Return type

[**crate::models::AnalysisStepList**](AnalysisStepList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cwl_input_json

> crate::models::CwlAnalysisInputJson get_cwl_input_json(project_id, analysis_id)
Retrieve the input json of a CWL analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**analysis_id** | **uuid::Uuid** | The ID of the CWL analysis for which to retrieve the input json | [required] |

### Return type

[**crate::models::CwlAnalysisInputJson**](CwlAnalysisInputJson.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cwl_output_json

> crate::models::CwlAnalysisOutputJson get_cwl_output_json(project_id, analysis_id)
Retrieve the output json of a CWL analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**analysis_id** | **uuid::Uuid** | The ID of the CWL analysis for which to retrieve the output json | [required] |

### Return type

[**crate::models::CwlAnalysisOutputJson**](CwlAnalysisOutputJson.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_analysis_output

> crate::models::AnalysisRawOutput get_raw_analysis_output(project_id, analysis_id)
Retrieve the raw output of an analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**analysis_id** | **String** | The ID of the analysis for which to retrieve the raw output | [required] |

### Return type

[**crate::models::AnalysisRawOutput**](AnalysisRawOutput.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_analysis

> crate::models::Analysis update_analysis(project_id, analysis_id, if_match, analysis)
Update an analysis.

Attributes which can be updated:    - tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**analysis_id** | **String** |  | [required] |
**if_match** | Option<**String**> | Optional header parameter to enable conflict exposure. If the client provides this header, then it must contains the client's most recent value of the 'ETag' response header, and the server will respond with a 409 code if it detects a conflict. If the client does not provide this header, then the server will not do a conflict check, which means that as a client you can override the resource even when the server has a more recent version. |  |
**analysis** | Option<[**Analysis**](Analysis.md)> |  |  |

### Return type

[**crate::models::Analysis**](Analysis.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

