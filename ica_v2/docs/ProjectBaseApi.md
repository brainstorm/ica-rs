# \ProjectBaseApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_base_connection_details**](ProjectBaseApi.md#create_base_connection_details) | **POST** /api/projects/{projectId}/base:connectionDetails | Creates the connection details to snowflake instance.
[**get_base_job**](ProjectBaseApi.md#get_base_job) | **GET** /api/projects/{projectId}/base/jobs/{baseJobId} | Retrieve a base job.
[**get_base_jobs**](ProjectBaseApi.md#get_base_jobs) | **GET** /api/projects/{projectId}/base/jobs | Retrieve a list of base jobs
[**get_base_tables**](ProjectBaseApi.md#get_base_tables) | **GET** /api/projects/{projectId}/base/tables | Retrieve a list of base tables.
[**load_data**](ProjectBaseApi.md#load_data) | **POST** /api/projects/{projectId}/base/tables/{tableId}:loadData | Load data in a base table.



## create_base_connection_details

> crate::models::BaseConnection create_base_connection_details(project_id)
Creates the connection details to snowflake instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**crate::models::BaseConnection**](BaseConnection.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_base_job

> crate::models::BaseJob get_base_job(project_id, base_job_id)
Retrieve a base job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**base_job_id** | **String** |  | [required] |

### Return type

[**crate::models::BaseJob**](BaseJob.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_base_jobs

> crate::models::BaseJobList get_base_jobs(project_id, page_offset, page_token, page_size, sort)
Retrieve a list of base jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\"  The attributes for which sorting is supported: - description - type |  |

### Return type

[**crate::models::BaseJobList**](BaseJobList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_base_tables

> crate::models::ProjectBaseTableList get_base_tables(project_id)
Retrieve a list of base tables.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**crate::models::ProjectBaseTableList**](ProjectBaseTableList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_data

> crate::models::BaseJob load_data(project_id, table_id, load_data_in_base_request)
Load data in a base table.

Load data in the specified table

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**table_id** | **String** |  | [required] |
**load_data_in_base_request** | Option<[**LoadDataInBaseRequest**](LoadDataInBaseRequest.md)> | Load data request |  |

### Return type

[**crate::models::BaseJob**](BaseJob.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

