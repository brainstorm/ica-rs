# \BundleDataApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bundle_data**](BundleDataApi.md#get_bundle_data) | **GET** /api/bundles/{bundleId}/data | Retrieve the list of bundle data.
[**link_data_to_bundle**](BundleDataApi.md#link_data_to_bundle) | **POST** /api/bundles/{bundleId}/data/{dataId} | Link data to this bundle.
[**unlink_data_from_bundle**](BundleDataApi.md#unlink_data_from_bundle) | **DELETE** /api/bundles/{bundleId}/data/{dataId} | Unlink data from this bundle.



## get_bundle_data

> crate::models::BundleDataPagedList get_bundle_data(bundle_id, full_text, id, filename, filename_match_mode, file_path, file_path_match_mode, status, format_id, format_code, r#type, parent_folder_id, parent_folder_path, creation_date_after, creation_date_before, status_date_after, status_date_before, user_tag, user_tag_match_mode, run_input_tag, run_input_tag_match_mode, run_output_tag, run_output_tag_match_mode, connector_tag, connector_tag_match_mode, technical_tag, technical_tag_match_mode, not_in_run, not_linked_to_sample, instrument_run_id, page_offset, page_token, page_size, sort)
Retrieve the list of bundle data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** |  | [required] |
**full_text** | Option<**String**> | To search through multiple fields of data. |  |
**id** | Option<**String**> | The ids to filter on. This will always match exact. |  |
**filename** | Option<**String**> | The filenames to filter on. The filenameMatchMode-parameter determines how the filtering is done. |  |
**filename_match_mode** | Option<**String**> | How the filenames are filtered. |  |
**file_path** | Option<**String**> | The paths of the files to filter on. |  |
**file_path_match_mode** | Option<**String**> | How the file paths are filtered:   - STARTS_WITH_CASE_INSENSITIVE: Filters the file path to start with the value of the 'filePath' parameter, regardless of upper/lower casing. This allows e.g. listing all data in a folder and all it's sub-folders (recursively).  - FULL_CASE_INSENSITIVE: Filters the file path to fully match the value of the 'filePath' parameter, regardless of upper/lower casing. Note that this can result in multiple results if e.g. two files exist with the same filename but different casing (abc.txt and ABC.txt). |  |[default to STARTS_WITH_CASE_INSENSITIVE]
**status** | Option<**String**> | The statuses to filter on. |  |
**format_id** | Option<**String**> | The IDs of the formats to filter on. |  |
**format_code** | Option<**String**> | The codes of the formats to filter on. |  |
**r#type** | Option<**String**> | The type to filter on. |  |
**parent_folder_id** | Option<**String**> | The IDs of parents folders to filter on. Lists all files and folders within the folder for the given ID, non-recursively. |  |
**parent_folder_path** | Option<**String**> | The full path of the parent folder. Should start and end with a '/'. Lists all files and folders within the folder for the given path, non-recursively. This can be used to browse through the hierarchical tree of folders, e.g. traversing one level up can be done by removing the last part of the path. |  |
**creation_date_after** | Option<**String**> | The date after which the data is created. Format: yyyy-MM-dd'T'HH:mm:ss'Z' eg: 2021-01-30T08:30:00Z |  |
**creation_date_before** | Option<**String**> | The date before which the data is created. Format: yyyy-MM-dd'T'HH:mm:ss'Z' eg: 2021-01-30T08:30:00Z |  |
**status_date_after** | Option<**String**> | The date after which the status has been updated. Format: yyyy-MM-dd'T'HH:mm:ss'Z' eg: 2021-01-30T08:30:00Z |  |
**status_date_before** | Option<**String**> | The date before which the status has been updated Format: yyyy-MM-dd'T'HH:mm:ss'Z' eg: 2021-01-30T08:30:00Z |  |
**user_tag** | Option<**String**> | The usertags to filter on. The userTagMatchMode-parameter determines how the filtering is done. |  |
**user_tag_match_mode** | Option<**String**> | How the usertags are filtered. |  |
**run_input_tag** | Option<**String**> | The runInputTags to filter on. The runInputTagMatchMode-parameter determines how the filtering is done. |  |
**run_input_tag_match_mode** | Option<**String**> | How the runInputTags are filtered. |  |
**run_output_tag** | Option<**String**> | The runOutputTags to filter on. The runOutputTagMatchMode-parameter determines how the filtering is done. |  |
**run_output_tag_match_mode** | Option<**String**> | How the runOutputTags are filtered. |  |
**connector_tag** | Option<**String**> | The connectorTags to filter on. The connectorTagMatchMode-parameter determines how the filtering is done. |  |
**connector_tag_match_mode** | Option<**String**> | How the connectorTags are filtered. |  |
**technical_tag** | Option<**String**> | The technicalTags to filter on. The techTagMatchMode-parameter determines how the filtering is done. |  |
**technical_tag_match_mode** | Option<**String**> | How the technicalTags are filtered. |  |
**not_in_run** | Option<**String**> | When set to true, the data will be filtered on data which is not used in a run. |  |
**not_linked_to_sample** | Option<**String**> | When set to true only date that is unlinked to a sample will be returned.  This filter implies a filter of type File. |  |
**instrument_run_id** | Option<[**Vec<String>**](String.md)> | The instrument run IDs of the sequencing runs to filter on. |  |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\"  The attributes for which sorting is supported: - timeCreated - timeModified - name - path - fileSizeInBytes - status - format - dataType - willBeArchivedAt - willBeDeletedAt |  |

### Return type

[**crate::models::BundleDataPagedList**](BundleDataPagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_data_to_bundle

> link_data_to_bundle(bundle_id, data_id)
Link data to this bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_data_from_bundle

> unlink_data_from_bundle(bundle_id, data_id)
Unlink data from this bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

