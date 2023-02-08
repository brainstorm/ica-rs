# \ProjectAnalysisCreationBatchApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_analysis_creation_batch**](ProjectAnalysisCreationBatchApi.md#create_analysis_creation_batch) | **POST** /api/projects/{projectId}/analysisCreationBatch | Create and start multiple analyses in batch.
[**get_analysis_creation_batch**](ProjectAnalysisCreationBatchApi.md#get_analysis_creation_batch) | **GET** /api/projects/{projectId}/analysisCreationBatch/{batchId} | Retrieve a analysis creation batch.
[**get_analysis_creation_batch_item**](ProjectAnalysisCreationBatchApi.md#get_analysis_creation_batch_item) | **GET** /api/projects/{projectId}/analysisCreationBatch/{batchId}/items | Retrieve a list of analysis creation batch items.
[**get_analysis_creation_batch_item1**](ProjectAnalysisCreationBatchApi.md#get_analysis_creation_batch_item1) | **GET** /api/projects/{projectId}/analysisCreationBatch/{batchId}/items/{itemId} | Retrieve a analysis creation batch item.



## create_analysis_creation_batch

> crate::models::AnalysisCreationBatch create_analysis_creation_batch(project_id, idempotency_key, create_analysis_creation_batch)
Create and start multiple analyses in batch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**idempotency_key** | Option<**String**> | The Idempotency-Key header can be used to prevent duplicate requests and support retries. It is implemented according to <a href=\"https://tools.ietf.org/id/draft-idempotency-header-01.html\">the IETF spec</a> and is allowed to be max 255 characters long. If the header is supplied, the response of the request will be saved for 7 days for the specific API endpoint, header value and user reference. When the same user makes a new request within 7 days to the same API endpoint with the same Idempotency-Key header value, following use cases can occur:<br /><ul><li>the request body is the same as the previous request and an answer is stored => the stored response is returned without executing the request again.</li><li>the request body is the same as the previous request and no answer is stored because the previous request has not finished => 409 error response.</li><li>the request body is not the same as the previous request => 422 error response.</li></ul>This means that each time when executing an API request using the Idempotency-Key header, the request has to contain a new value that hasn't been used in the past 7 days for that specific API and by the specific user. |  |
**create_analysis_creation_batch** | Option<[**CreateAnalysisCreationBatch**](CreateAnalysisCreationBatch.md)> |  |  |

### Return type

[**crate::models::AnalysisCreationBatch**](AnalysisCreationBatch.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analysis_creation_batch

> crate::models::AnalysisCreationBatch get_analysis_creation_batch(project_id, batch_id)
Retrieve a analysis creation batch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**batch_id** | **uuid::Uuid** | The ID of the analysis creation batch | [required] |

### Return type

[**crate::models::AnalysisCreationBatch**](AnalysisCreationBatch.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analysis_creation_batch_item

> crate::models::AnalysisCreationBatchItemPagedList get_analysis_creation_batch_item(project_id, batch_id, status, page_offset, page_token, page_size, sort)
Retrieve a list of analysis creation batch items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**batch_id** | **uuid::Uuid** | The ID of the analysis creation batch | [required] |
**status** | Option<[**Vec<String>**](String.md)> | The statuses to filter on. |  |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\" |  |

### Return type

[**crate::models::AnalysisCreationBatchItemPagedList**](AnalysisCreationBatchItemPagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analysis_creation_batch_item1

> crate::models::AnalysisCreationBatchItem get_analysis_creation_batch_item1(project_id, batch_id, item_id)
Retrieve a analysis creation batch item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**batch_id** | **uuid::Uuid** | The ID of the analysis creation batch | [required] |
**item_id** | **uuid::Uuid** | The ID of the analysis creation batch item | [required] |

### Return type

[**crate::models::AnalysisCreationBatchItem**](AnalysisCreationBatchItem.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

