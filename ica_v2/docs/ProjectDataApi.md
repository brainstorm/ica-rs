# \ProjectDataApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_secondary_data**](ProjectDataApi.md#add_secondary_data) | **POST** /api/projects/{projectId}/data/{dataId}/secondaryData/{secondaryDataId} | Add secondary data to data.
[**archive_data**](ProjectDataApi.md#archive_data) | **POST** /api/projects/{projectId}/data/{dataId}:archive | Schedule this data for archival.
[**complete_folder_upload_session**](ProjectDataApi.md#complete_folder_upload_session) | **POST** /api/projects/{projectId}/data/{dataId}/folderUploadSessions/{folderUploadSessionId}:complete | Complete a trackable folder upload session.
[**create_data_in_project**](ProjectDataApi.md#create_data_in_project) | **POST** /api/projects/{projectId}/data | Create data in this project.
[**create_download_url_for_data**](ProjectDataApi.md#create_download_url_for_data) | **POST** /api/projects/{projectId}/data/{dataId}:createDownloadUrl | Retrieve a download URL for this data.
[**create_download_urls_for_data**](ProjectDataApi.md#create_download_urls_for_data) | **POST** /api/projects/{projectId}/data:createDownloadUrls | Retrieve download URLs for the data.
[**create_folder_upload_session**](ProjectDataApi.md#create_folder_upload_session) | **POST** /api/projects/{projectId}/data/{dataId}/folderUploadSessions | Create a trackable folder upload session.
[**create_inline_view_url_for_data**](ProjectDataApi.md#create_inline_view_url_for_data) | **POST** /api/projects/{projectId}/data/{dataId}:createInlineViewUrl | Retrieve an URL for this data to use for inline view in a browser.
[**create_temporary_credentials_for_data**](ProjectDataApi.md#create_temporary_credentials_for_data) | **POST** /api/projects/{projectId}/data/{dataId}:createTemporaryCredentials | Retrieve temporary credentials for this data.
[**create_upload_url_for_data**](ProjectDataApi.md#create_upload_url_for_data) | **POST** /api/projects/{projectId}/data/{dataId}:createUploadUrl | Retrieve an upload URL for this data.
[**delete_data**](ProjectDataApi.md#delete_data) | **POST** /api/projects/{projectId}/data/{dataId}:delete | Schedule this data for deletion.
[**get_data_eligible_for_linking**](ProjectDataApi.md#get_data_eligible_for_linking) | **GET** /api/projects/{projectId}/data/eligibleForLinking | Retrieve a list of data eligible for linking to the current project.
[**get_folder_upload_session**](ProjectDataApi.md#get_folder_upload_session) | **GET** /api/projects/{projectId}/data/{dataId}/folderUploadSessions/{folderUploadSessionId} | Retrieve folder upload session details.
[**get_non_sample_project_data**](ProjectDataApi.md#get_non_sample_project_data) | **GET** /api/projects/{projectId}/data/nonSampleData | Retrieve a list of project data not linked to a sample.
[**get_project_data**](ProjectDataApi.md#get_project_data) | **GET** /api/projects/{projectId}/data/{dataId} | Retrieve a project data.
[**get_project_data_children**](ProjectDataApi.md#get_project_data_children) | **GET** /api/projects/{projectId}/data/{dataId}/children | Retrieve the children of this data.
[**get_project_data_list**](ProjectDataApi.md#get_project_data_list) | **GET** /api/projects/{projectId}/data | Retrieve the list of project data.
[**get_projects_linked_to_data**](ProjectDataApi.md#get_projects_linked_to_data) | **GET** /api/projects/{projectId}/data/{dataId}/linkedProjects | Retrieve a list of projects to which this data is linked.
[**get_secondary_data**](ProjectDataApi.md#get_secondary_data) | **GET** /api/projects/{projectId}/data/{dataId}/secondaryData | Retrieve a list of secondary data for data.
[**link_data_to_project**](ProjectDataApi.md#link_data_to_project) | **POST** /api/projects/{projectId}/data/{dataId} | Link data to this project.
[**remove_secondary_data**](ProjectDataApi.md#remove_secondary_data) | **DELETE** /api/projects/{projectId}/data/{dataId}/secondaryData/{secondaryDataId} | Remove secondary data from data.
[**schedule_download_for_data**](ProjectDataApi.md#schedule_download_for_data) | **POST** /api/projects/{projectId}/data/{dataId}:scheduleDownload | Schedule a download.
[**unarchive_data**](ProjectDataApi.md#unarchive_data) | **POST** /api/projects/{projectId}/data/{dataId}:unarchive | Schedule this data for unarchival.
[**unlink_data_from_project**](ProjectDataApi.md#unlink_data_from_project) | **POST** /api/projects/{projectId}/data/{dataId}:unlink | Unlink data from this project.
[**update_project_data**](ProjectDataApi.md#update_project_data) | **PUT** /api/projects/{projectId}/data/{dataId} | Update this project data.



## add_secondary_data

> add_secondary_data(project_id, data_id, secondary_data_id)
Add secondary data to data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |
**secondary_data_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_data

> archive_data(project_id, data_id)
Schedule this data for archival.

Endpoint for scheduling this data for archival. This will also archive all files and directories below that data.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_folder_upload_session

> crate::models::FolderUploadSession complete_folder_upload_session(project_id, data_id, folder_upload_session_id, complete_folder_upload_session)
Complete a trackable folder upload session.

Complete a trackable folder upload session. By completing the folder upload session, and specifying how many files you have uploaded, ICA can ensure that all uploaded files are accounted for.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |
**folder_upload_session_id** | **String** |  | [required] |
**complete_folder_upload_session** | Option<[**CompleteFolderUploadSession**](CompleteFolderUploadSession.md)> | The info required to complete the folder upload session. |  |

### Return type

[**crate::models::FolderUploadSession**](FolderUploadSession.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_data_in_project

> crate::models::ProjectData create_data_in_project(project_id, create_data)
Create data in this project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**create_data** | Option<[**CreateData**](CreateData.md)> | The data to create. |  |

### Return type

[**crate::models::ProjectData**](ProjectData.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_download_url_for_data

> crate::models::Download create_download_url_for_data(project_id, data_id)
Retrieve a download URL for this data.

Can be used to download a file directly from the region where it is located, no connector is needed. Not applicable for Folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

[**crate::models::Download**](Download.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_download_urls_for_data

> crate::models::DataUrlList create_download_urls_for_data(project_id, data_url_id_list)
Retrieve download URLs for the data.

Can be used to download files directly from the region where it is located, no connector is needed. Not applicable for Folders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_url_id_list** | Option<[**DataUrlIdList**](DataUrlIdList.md)> |  |  |

### Return type

[**crate::models::DataUrlList**](DataUrlList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_folder_upload_session

> crate::models::FolderUploadSession create_folder_upload_session(project_id, data_id, create_temporary_credentials)
Create a trackable folder upload session.

This endpoint can be used to ensure that all uploaded files within the requested session are accounted for. This call has to be used together with the :complete endpoint once upload is done.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |
**create_temporary_credentials** | Option<[**CreateTemporaryCredentials**](CreateTemporaryCredentials.md)> | Temporary credentials request options. |  |

### Return type

[**crate::models::FolderUploadSession**](FolderUploadSession.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inline_view_url_for_data

> crate::models::InlineView create_inline_view_url_for_data(project_id, data_id)
Retrieve an URL for this data to use for inline view in a browser.

Can be used to view a file directly from the region where it is located, no connector is needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineView**](InlineView.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_temporary_credentials_for_data

> crate::models::TempCredentials create_temporary_credentials_for_data(project_id, data_id, create_temporary_credentials)
Retrieve temporary credentials for this data.

Can be used to upload or download a file directly from the region where it is located, no connector is needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |
**create_temporary_credentials** | Option<[**CreateTemporaryCredentials**](CreateTemporaryCredentials.md)> | Temporary credentials request options. |  |

### Return type

[**crate::models::TempCredentials**](TempCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_upload_url_for_data

> crate::models::Upload create_upload_url_for_data(project_id, data_id, file_type, hash)
Retrieve an upload URL for this data.

Can be used to upload a file directly from the region where it is located, no connector is needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |
**file_type** | Option<**String**> |  |  |
**hash** | Option<**String**> |  |  |

### Return type

[**crate::models::Upload**](Upload.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data

> delete_data(project_id, data_id)
Schedule this data for deletion.

Endpoint for scheduling this data for deletion. This will also delete all files and directories below that data.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_eligible_for_linking

> crate::models::DataPagedList get_data_eligible_for_linking(project_id, full_text, id, filename, filename_match_mode, file_path, file_path_match_mode, status, format_id, format_code, r#type, parent_folder_id, parent_folder_path, creation_date_after, creation_date_before, status_date_after, status_date_before, user_tag, user_tag_match_mode, run_input_tag, run_input_tag_match_mode, run_output_tag, run_output_tag_match_mode, connector_tag, connector_tag_match_mode, technical_tag, technical_tag_match_mode, not_in_run, not_linked_to_sample, instrument_run_id, page_offset, page_token, page_size, sort)
Retrieve a list of data eligible for linking to the current project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
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
**not_linked_to_sample** | Option<**bool**> | When set to true only data that is unlinked to a sample will be returned. This filter implies a filter of type File. |  |
**instrument_run_id** | Option<[**Vec<String>**](String.md)> | The instrument run IDs of the sequencing runs to filter on. |  |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\"  The attributes for which sorting is supported: - timeCreated - timeModified - name - path - fileSizeInBytes - status - format - dataType - willBeArchivedAt - willBeDeletedAt |  |

### Return type

[**crate::models::DataPagedList**](DataPagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folder_upload_session

> crate::models::FolderUploadSession get_folder_upload_session(project_id, data_id, folder_upload_session_id)
Retrieve folder upload session details.

Retrieve folder upload session details, including the current status of your upload session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |
**folder_upload_session_id** | **String** |  | [required] |

### Return type

[**crate::models::FolderUploadSession**](FolderUploadSession.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_non_sample_project_data

> crate::models::ProjectDataPagedList get_non_sample_project_data(project_id, page_offset, page_token, page_size)
Retrieve a list of project data not linked to a sample.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |

### Return type

[**crate::models::ProjectDataPagedList**](ProjectDataPagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_data

> crate::models::ProjectData get_project_data(project_id, data_id)
Retrieve a project data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

[**crate::models::ProjectData**](ProjectData.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_data_children

> crate::models::ProjectDataPagedList get_project_data_children(project_id, data_id, page_offset, page_token, page_size)
Retrieve the children of this data.

# Changelog For this endpoint multiple versions exist. Note that the values for request headers 'Content-Type' and 'Accept' must contain a matching version.  ## [V3] Initial version ## [V4] Added pagination 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |

### Return type

[**crate::models::ProjectDataPagedList**](ProjectDataPagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v4+json, application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_data_list

> crate::models::ProjectDataPagedList get_project_data_list(project_id, full_text, id, filename, filename_match_mode, file_path, file_path_match_mode, status, format_id, format_code, r#type, parent_folder_id, parent_folder_path, creation_date_after, creation_date_before, status_date_after, status_date_before, user_tag, user_tag_match_mode, run_input_tag, run_input_tag_match_mode, run_output_tag, run_output_tag_match_mode, connector_tag, connector_tag_match_mode, technical_tag, technical_tag_match_mode, not_in_run, not_linked_to_sample, instrument_run_id, page_offset, page_token, page_size, sort)
Retrieve the list of project data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
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
**not_linked_to_sample** | Option<**bool**> | When set to true only data that is unlinked to a sample will be returned.  This filter implies a filter of type File. |  |
**instrument_run_id** | Option<[**Vec<String>**](String.md)> | The instrument run IDs of the sequencing runs to filter on. |  |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\"  The attributes for which sorting is supported: - timeCreated - timeModified - name - path - fileSizeInBytes - status - format - dataType - willBeArchivedAt - willBeDeletedAt |  |

### Return type

[**crate::models::ProjectDataPagedList**](ProjectDataPagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_linked_to_data

> crate::models::ProjectList get_projects_linked_to_data(project_id, data_id)
Retrieve a list of projects to which this data is linked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

[**crate::models::ProjectList**](ProjectList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_secondary_data

> crate::models::DataList get_secondary_data(project_id, data_id)
Retrieve a list of secondary data for data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

[**crate::models::DataList**](DataList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_data_to_project

> crate::models::ProjectData link_data_to_project(project_id, data_id)
Link data to this project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

[**crate::models::ProjectData**](ProjectData.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_secondary_data

> remove_secondary_data(project_id, data_id, secondary_data_id)
Remove secondary data from data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |
**secondary_data_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_download_for_data

> crate::models::DataTransfer schedule_download_for_data(project_id, data_id, schedule_download)
Schedule a download.

Endpoint for scheduling a download for the data specified by the ID to a connector. This download will only start when the connector is running. This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |
**schedule_download** | [**ScheduleDownload**](ScheduleDownload.md) |  | [required] |

### Return type

[**crate::models::DataTransfer**](DataTransfer.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unarchive_data

> unarchive_data(project_id, data_id)
Schedule this data for unarchival.

Endpoint for scheduling this data for unarchival. This will also unarchive all files and directories below that data. This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_data_from_project

> unlink_data_from_project(project_id, data_id)
Unlink data from this project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_data

> crate::models::ProjectData update_project_data(project_id, data_id, project_data)
Update this project data.

Fields which can be updated for files:  - data.willBeArchivedAt  - data.willBeDeletedAt  - data.format  - data.tags  Fields which can be updated for folders:  - data.tags  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**data_id** | **String** |  | [required] |
**project_data** | Option<[**ProjectData**](ProjectData.md)> | The updated project data. |  |

### Return type

[**crate::models::ProjectData**](ProjectData.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

