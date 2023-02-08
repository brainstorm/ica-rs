# \ProjectDataLinkingBatchApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_data_linking_batch**](ProjectDataLinkingBatchApi.md#create_project_data_linking_batch) | **POST** /api/projects/{projectId}/dataLinkingBatch | Create a project data linking batch.
[**get_project_data_linking_batch**](ProjectDataLinkingBatchApi.md#get_project_data_linking_batch) | **GET** /api/projects/{projectId}/dataLinkingBatch/{batchId} | Retrieve a project data linking batch.
[**get_project_data_linking_batch_item**](ProjectDataLinkingBatchApi.md#get_project_data_linking_batch_item) | **GET** /api/projects/{projectId}/dataLinkingBatch/{batchId}/items/{itemId} | Retrieve a project data linking batch item.
[**get_project_data_linking_batch_items**](ProjectDataLinkingBatchApi.md#get_project_data_linking_batch_items) | **GET** /api/projects/{projectId}/dataLinkingBatch/{batchId}/items | Retrieve a list of project data linking batch items.



## create_project_data_linking_batch

> crate::models::ProjectDataLinkingBatch create_project_data_linking_batch(project_id, create_project_data_linking_batch)
Create a project data linking batch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**create_project_data_linking_batch** | Option<[**CreateProjectDataLinkingBatch**](CreateProjectDataLinkingBatch.md)> |  |  |

### Return type

[**crate::models::ProjectDataLinkingBatch**](ProjectDataLinkingBatch.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_data_linking_batch

> crate::models::ProjectDataLinkingBatch get_project_data_linking_batch(project_id, batch_id)
Retrieve a project data linking batch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**batch_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::ProjectDataLinkingBatch**](ProjectDataLinkingBatch.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_data_linking_batch_item

> crate::models::ProjectDataLinkingBatchItem get_project_data_linking_batch_item(project_id, batch_id, item_id)
Retrieve a project data linking batch item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**batch_id** | **uuid::Uuid** |  | [required] |
**item_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::ProjectDataLinkingBatchItem**](ProjectDataLinkingBatchItem.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_data_linking_batch_items

> crate::models::ProjectDataLinkingBatchItemPagedList get_project_data_linking_batch_items(project_id, batch_id, status, page_offset, page_token, page_size, sort)
Retrieve a list of project data linking batch items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**batch_id** | **uuid::Uuid** |  | [required] |
**status** | Option<[**Vec<String>**](String.md)> | The statuses to filter on. |  |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\" |  |

### Return type

[**crate::models::ProjectDataLinkingBatchItemPagedList**](ProjectDataLinkingBatchItemPagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

