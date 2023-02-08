# \ProjectSampleApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_metadata_model_to_sample**](ProjectSampleApi.md#add_metadata_model_to_sample) | **POST** /api/projects/{projectId}/samples/{sampleId}/metadata/{metadataModelId} | Add a metadata model to a sample.
[**complete_project_sample**](ProjectSampleApi.md#complete_project_sample) | **POST** /api/projects/{projectId}/samples/{sampleId}:complete | Completes the sample after data has been linked to it.
[**create_sample_in_project**](ProjectSampleApi.md#create_sample_in_project) | **POST** /api/projects/{projectId}/samples | Create a new sample in this project
[**deep_delete_sample**](ProjectSampleApi.md#deep_delete_sample) | **POST** /api/projects/{projectId}/samples/{sampleId}:deleteDeep | Delete a sample together with all of its data.
[**delete_and_unlink_sample**](ProjectSampleApi.md#delete_and_unlink_sample) | **POST** /api/projects/{projectId}/samples/{sampleId}:deleteUnlink | Delete a sample and unlink its data.
[**delete_sample_with_input**](ProjectSampleApi.md#delete_sample_with_input) | **POST** /api/projects/{projectId}/samples/{sampleId}:deleteWithInput | Delete a sample as well as its input data.
[**get_project_sample**](ProjectSampleApi.md#get_project_sample) | **GET** /api/projects/{projectId}/samples/{sampleId} | Retrieve a project sample.
[**get_project_samples**](ProjectSampleApi.md#get_project_samples) | **POST** /api/projects/{projectId}/samples:search | Retrieve project samples.
[**get_projects_for_sample**](ProjectSampleApi.md#get_projects_for_sample) | **GET** /api/projects/{projectId}/samples/{sampleId}/projects | Retrieve a list of projects for this sample.
[**get_sample_data_list**](ProjectSampleApi.md#get_sample_data_list) | **GET** /api/projects/{projectId}/samples/{sampleId}/data | Retrieve the list of sample data.
[**get_sample_history**](ProjectSampleApi.md#get_sample_history) | **GET** /api/projects/{projectId}/samples/{sampleId}/history | Retrieve sample history.
[**get_sample_metadata_field**](ProjectSampleApi.md#get_sample_metadata_field) | **GET** /api/projects/{projectId}/samples/{sampleId}/metadata/field/{fieldId} | Retrieve a metadata field.
[**get_sample_metadata_field_count**](ProjectSampleApi.md#get_sample_metadata_field_count) | **GET** /api/projects/{projectId}/samples/{sampleId}/metadata/{fieldId}/fieldCount | Retrieves the number of occurrences of a given field.
[**link_data_to_sample**](ProjectSampleApi.md#link_data_to_sample) | **POST** /api/projects/{projectId}/samples/{sampleId}/data/{dataId} | Link data to a sample.
[**link_sample_to_project**](ProjectSampleApi.md#link_sample_to_project) | **POST** /api/projects/{projectId}/samples/{sampleId} | Link a sample to a project.
[**mark_sample_deleted**](ProjectSampleApi.md#mark_sample_deleted) | **POST** /api/projects/{projectId}/samples/{sampleId}:deleteMark | Mark a sample deleted.
[**unlink_data_from_sample**](ProjectSampleApi.md#unlink_data_from_sample) | **POST** /api/projects/{projectId}/samples/{sampleId}/data/{dataId}:unlink | Unlink data from a sample.
[**unlink_sample_from_project**](ProjectSampleApi.md#unlink_sample_from_project) | **POST** /api/projects/{projectId}/samples/{sampleId}:unlink | Unlink a sample from a project.
[**update_project_sample**](ProjectSampleApi.md#update_project_sample) | **PUT** /api/projects/{projectId}/samples/{sampleId} | Update a project sample.
[**update_sample_metadata_fields**](ProjectSampleApi.md#update_sample_metadata_fields) | **POST** /api/projects/{projectId}/samples/{sampleId}/metadata:updateFields | Update metadata fields.



## add_metadata_model_to_sample

> add_metadata_model_to_sample(project_id, sample_id, metadata_model_id)
Add a metadata model to a sample.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |
**metadata_model_id** | **String** | The ID of the metadata model | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_project_sample

> complete_project_sample(project_id, sample_id)
Completes the sample after data has been linked to it.

Completes the sample after data has been linked to it. The sample status will be set to 'Available' and a sample completed event will be triggered as well.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sample_in_project

> crate::models::ProjectSample create_sample_in_project(project_id, create_sample)
Create a new sample in this project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**create_sample** | Option<[**CreateSample**](CreateSample.md)> |  |  |

### Return type

[**crate::models::ProjectSample**](ProjectSample.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deep_delete_sample

> deep_delete_sample(project_id, sample_id)
Delete a sample together with all of its data.

Endpoint deleting a sample together with all of its data.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_and_unlink_sample

> delete_and_unlink_sample(project_id, sample_id)
Delete a sample and unlink its data.

Endpoint for deleting a sample while unlinking its data.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sample_with_input

> delete_sample_with_input(project_id, sample_id)
Delete a sample as well as its input data.

Endpoint for deleting a sample as well as its input data.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_sample

> crate::models::ProjectSample get_project_sample(project_id, sample_id)
Retrieve a project sample.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |

### Return type

[**crate::models::ProjectSample**](ProjectSample.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_samples

> crate::models::ProjectSamplePagedList get_project_samples(project_id, page_offset, page_token, page_size, sort, find_project_samples)
Retrieve project samples.

Endpoint for retrieving project samples. This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\" The attributes for which sorting is supported: - timeCreated - timeModified - name - description - metadataValid - status |  |
**find_project_samples** | Option<[**FindProjectSamples**](FindProjectSamples.md)> |  |  |

### Return type

[**crate::models::ProjectSamplePagedList**](ProjectSamplePagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_for_sample

> crate::models::ProjectList get_projects_for_sample(project_id, sample_id)
Retrieve a list of projects for this sample.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |

### Return type

[**crate::models::ProjectList**](ProjectList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sample_data_list

> crate::models::DataList get_sample_data_list(project_id, sample_id, full_text, id, filename, filename_match_mode, file_path, file_path_match_mode, status, format_id, format_code, r#type, parent_folder_id, parent_folder_path, creation_date_after, creation_date_before, status_date_after, status_date_before, user_tag, user_tag_match_mode, run_input_tag, run_input_tag_match_mode, run_output_tag, run_output_tag_match_mode, connector_tag, connector_tag_match_mode, technical_tag, technical_tag_match_mode, not_in_run, instrument_run_id, page_offset, page_token, page_size, sort)
Retrieve the list of sample data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample to retrieve data for | [required] |
**full_text** | Option<**String**> | To search through multiple fields of data. |  |
**id** | Option<[**Vec<String>**](String.md)> | The ids to filter on. This will always match exact. |  |
**filename** | Option<[**Vec<String>**](String.md)> | The filenames to filter on. The filenameMatchMode-parameter determines how the filtering is done. |  |
**filename_match_mode** | Option<**String**> | How the filenames are filtered.  |  |
**file_path** | Option<[**Vec<String>**](String.md)> | The paths of the files to filter on. |  |
**file_path_match_mode** | Option<**String**> | How the file paths are filtered:   - STARTS_WITH_CASE_INSENSITIVE: Filters the file path to start with the value of the 'filePath' parameter, regardless of upper/lower casing. This allows e.g. listing all data in a folder and all it's sub-folders (recursively).  - FULL_CASE_INSENSITIVE: Filters the file path to fully match the value of the 'filePath' parameter, regardless of upper/lower casing. Note that this can result in multiple results if e.g. two files exist with the same filename but different casing (abc.txt and ABC.txt). |  |[default to STARTS_WITH_CASE_INSENSITIVE]
**status** | Option<[**Vec<String>**](String.md)> | The statuses to filter on. |  |
**format_id** | Option<[**Vec<String>**](String.md)> | The IDs of the formats to filter on. |  |
**format_code** | Option<[**Vec<String>**](String.md)> | The codes of the formats to filter on. |  |
**r#type** | Option<**String**> | The type to filter on. |  |
**parent_folder_id** | Option<[**Vec<String>**](String.md)> | The IDs of parents folders to filter on. Lists all files and folders within the folder for the given ID, non-recursively. |  |
**parent_folder_path** | Option<**String**> | The full path of the parent folder. Should start and end with a '/'. Lists all files and folders within the folder for the given path, non-recursively. This can be used to browse through the hierarchical tree of folders, e.g. traversing one level up can be done by removing the last part of the path. |  |
**creation_date_after** | Option<**String**> | The date after which the data is created. Format: yyyy-MM-dd'T'HH:mm:ss'Z' eg: 2021-01-30T08:30:00Z |  |
**creation_date_before** | Option<**String**> | The date before which the data is created. Format: yyyy-MM-dd'T'HH:mm:ss'Z' eg: 2021-01-30T08:30:00Z |  |
**status_date_after** | Option<**String**> | The date after which the status has been updated. Format: yyyy-MM-dd'T'HH:mm:ss'Z' eg: 2021-01-30T08:30:00Z |  |
**status_date_before** | Option<**String**> | The date before which the status has been updated. Format: yyyy-MM-dd'T'HH:mm:ss'Z' eg: 2021-01-30T08:30:00Z |  |
**user_tag** | Option<[**Vec<String>**](String.md)> | The usertags to filter on. The userTagMatchMode-parameter determines how the filtering is done. |  |
**user_tag_match_mode** | Option<**String**> | How the usertags are filtered.  |  |
**run_input_tag** | Option<[**Vec<String>**](String.md)> | The runInputTags to filter on. The runInputTagMatchMode-parameter determines how the filtering is done. |  |
**run_input_tag_match_mode** | Option<**String**> | How the runInputTags are filtered.  |  |
**run_output_tag** | Option<[**Vec<String>**](String.md)> | The runOutputTags to filter on. The runOutputTagMatchMode-parameter determines how the filtering is done. |  |
**run_output_tag_match_mode** | Option<**String**> | How the runOutputTags are filtered.  |  |
**connector_tag** | Option<[**Vec<String>**](String.md)> | The connectorTags to filter on. The connectorTagMatchMode-parameter determines how the filtering is done. |  |
**connector_tag_match_mode** | Option<**String**> | How the connectorTags are filtered.  |  |
**technical_tag** | Option<[**Vec<String>**](String.md)> | The technicalTags to filter on. The techTagMatchMode-parameter determines how the filtering is done. |  |
**technical_tag_match_mode** | Option<**String**> | How the technicalTags are filtered.  |  |
**not_in_run** | Option<**bool**> | When set to true, the data will be filtered on data which is not used in a run. |  |
**instrument_run_id** | Option<[**Vec<String>**](String.md)> | The instrument run IDs of the sequencing runs to filter on. |  |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\"  The attributes for which sorting is supported: - timeCreated - timeModified - name - path - fileSizeInBytes - status - format - dataType - willBeArchivedAt - willBeDeletedAt |  |

### Return type

[**crate::models::DataList**](DataList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sample_history

> crate::models::SampleHistoryList get_sample_history(project_id, sample_id)
Retrieve sample history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |

### Return type

[**crate::models::SampleHistoryList**](SampleHistoryList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sample_metadata_field

> crate::models::Field get_sample_metadata_field(project_id, sample_id, field_id)
Retrieve a metadata field.

Returns a list of values for the field with identifier fieldId belonging to the sample with identifier sampleId. If the field is a group field that can occur more than once or belongs to a group field that can occur more than once the return value will have one entry in the list for each occurrence. If not the return value will be a single value list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |
**field_id** | **String** | The ID of the field | [required] |

### Return type

[**crate::models::Field**](Field.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sample_metadata_field_count

> crate::models::Field get_sample_metadata_field_count(project_id, sample_id, field_id)
Retrieves the number of occurrences of a given field.

Returns a list of values for the field with identifier fieldId belonging to the sample with identifier sampleId. If the field is a group field that can occur more than once or belongs to a group field that can occur more than once the return value will have one entry in the list for each occurrence. If not the return value will be a single value list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |
**field_id** | **String** | The ID of the field | [required] |

### Return type

[**crate::models::Field**](Field.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_data_to_sample

> link_data_to_sample(project_id, sample_id, data_id)
Link data to a sample.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |
**data_id** | **String** | The ID of the data to link | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_sample_to_project

> crate::models::ProjectSample link_sample_to_project(project_id, sample_id)
Link a sample to a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** |  | [required] |

### Return type

[**crate::models::ProjectSample**](ProjectSample.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_sample_deleted

> mark_sample_deleted(project_id, sample_id)
Mark a sample deleted.

Endpoint for marking a sample as deleted.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_data_from_sample

> unlink_data_from_sample(project_id, sample_id, data_id)
Unlink data from a sample.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** | The ID of the sample | [required] |
**data_id** | **String** | The ID of the data to unlink | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_sample_from_project

> unlink_sample_from_project(project_id, sample_id)
Unlink a sample from a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_sample

> crate::models::ProjectSample update_project_sample(project_id, sample_id, if_match, project_sample)
Update a project sample.

Fields which can be updated: - sample.name - sample.description - sample.status - sample.tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** |  | [required] |
**if_match** | Option<**String**> | Optional header parameter to enable conflict exposure. If the client provides this header, then it must contains the client's most recent value of the 'ETag' response header, and the server will respond with a 409 code if it detects a conflict. If the client does not provide this header, then the server will not do a conflict check, which means that as a client you can override the resource even when the server has a more recent version. |  |
**project_sample** | Option<[**ProjectSample**](ProjectSample.md)> |  |  |

### Return type

[**crate::models::ProjectSample**](ProjectSample.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sample_metadata_fields

> crate::models::Sample update_sample_metadata_fields(project_id, sample_id, update_metadata)
Update metadata fields.

Endpoint for updating metadata fields.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**sample_id** | **String** |  | [required] |
**update_metadata** | Option<[**UpdateMetadata**](UpdateMetadata.md)> |  |  |

### Return type

[**crate::models::Sample**](Sample.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

