# \FoldersApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_folder**](FoldersApi.md#archive_folder) | **POST** /v1/folders/{folderId}:archive | Archive a folder
[**bulk_folder_update**](FoldersApi.md#bulk_folder_update) | **PATCH** /v1/folders | Updates list of folders with metadata
[**complete_folder_session**](FoldersApi.md#complete_folder_session) | **POST** /v1/folders/{folderId}/sessions/{sessionId}:complete | Complete a folder upload in GDS
[**copy_folder**](FoldersApi.md#copy_folder) | **POST** /v1/folders/{folderId}:copy | Copy a folder
[**create_folder**](FoldersApi.md#create_folder) | **POST** /v1/folders | Create a folder in GDS and receive credentials for upload
[**create_folder_session**](FoldersApi.md#create_folder_session) | **POST** /v1/folders/{folderId}/sessions | Create a session
[**delete_folder**](FoldersApi.md#delete_folder) | **DELETE** /v1/folders/{folderId} | Deletes a folder by id
[**get_folder**](FoldersApi.md#get_folder) | **GET** /v1/folders/{folderId} | Get information about a folder in GDS.
[**get_folder_job**](FoldersApi.md#get_folder_job) | **GET** /v1/folders/{folderId}/jobs/{jobId} | Get status of a folder job in GDS
[**get_folder_session**](FoldersApi.md#get_folder_session) | **GET** /v1/folders/{folderId}/sessions/{sessionId} | Get status of a folder upload in GDS
[**list_folders**](FoldersApi.md#list_folders) | **GET** /v1/folders | Get a list of folders
[**unarchive_folder**](FoldersApi.md#unarchive_folder) | **POST** /v1/folders/{folderId}:unarchive | Unarchive a folder
[**update_folder**](FoldersApi.md#update_folder) | **PATCH** /v1/folders/{folderId} | Update a folder content or acl



## archive_folder

> crate::models::FolderResponse archive_folder(folder_id, body)
Archive a folder

Archives a folder to a lower storage cost tier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Unique identifier for the folder to be archived. | [required] |
**body** | [**FolderArchiveRequest**](FolderArchiveRequest.md) |  | [required] |

### Return type

[**crate::models::FolderResponse**](FolderResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_folder_update

> crate::models::BulkFolderUpdateResponse bulk_folder_update(body)
Updates list of folders with metadata

Updates list of folders with metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**BulkFolderUpdateRequest**](BulkFolderUpdateRequest.md)> |  |  |

### Return type

[**crate::models::BulkFolderUpdateResponse**](BulkFolderUpdateResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_folder_session

> crate::models::SessionResponse complete_folder_session(folder_id, session_id, body)
Complete a folder upload in GDS

Complete a folder upload in GDS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Unique identifier for the folder related to the upload session. | [required] |
**session_id** | **String** | The id of the upload session | [required] |
**body** | [**CompleteSessionRequest**](CompleteSessionRequest.md) | The request body | [required] |

### Return type

[**crate::models::SessionResponse**](SessionResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## copy_folder

> crate::models::JobResponse copy_folder(folder_id, body, tenant_id)
Copy a folder

Copy a folder into a target parent folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Unique identifier for the folder to be copied. | [required] |
**body** | [**FolderCopyRequest**](FolderCopyRequest.md) |  | [required] |
**tenant_id** | Option<**String**> | Optional parameter to copy from a shared folder in another tenant |  |

### Return type

[**crate::models::JobResponse**](JobResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_folder

> crate::models::FolderWriteableResponse create_folder(body, include)
Create a folder in GDS and receive credentials for upload

Create a folder entry in GDS. Returns temporary credentials for folder upload directly to S3 when the include=objectStoreAccess parameter is used. Volume ID or volume name is required for folder creation. If a folder path is provided and does not exist, GDS automatically creates the folder path in the appropriate account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateFolderRequest**](CreateFolderRequest.md) |  | [required] |
**include** | Option<**String**> | Optionally include additional fields in the response.              Possible values: ObjectStoreAccess |  |

### Return type

[**crate::models::FolderWriteableResponse**](FolderWriteableResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_folder_session

> crate::models::CreateSessionResponse create_folder_session(folder_id, body)
Create a session

Create a session and credentials used for accessing the object store directly

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** |  | [required] |
**body** | [**CreateSessionRequest**](CreateSessionRequest.md) |  | [required] |

### Return type

[**crate::models::CreateSessionResponse**](CreateSessionResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_folder

> crate::models::FolderResponse delete_folder(folder_id)
Deletes a folder by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Unique identifier for the folder to be deleted. | [required] |

### Return type

[**crate::models::FolderResponse**](FolderResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folder

> crate::models::FolderResponse get_folder(folder_id, tenant_id, include_volume_metadata, metadata_include, metadata_exclude)
Get information about a folder in GDS.

Get information for the specified folder ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Unique identifier for the folder to retrieve. | [required] |
**tenant_id** | Option<**String**> | Optional parameter to see shared data in another tenant |  |
**include_volume_metadata** | Option<**bool**> | Optional parameter to return volume's metadata |  |
**metadata_include** | Option<**String**> | Optional parameter to specify comma separated patterns to include metadata by their field names. |  |
**metadata_exclude** | Option<**String**> | Optional parameter to specify comma separated patterns to exclude metadata by their field names. |  |

### Return type

[**crate::models::FolderResponse**](FolderResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folder_job

> crate::models::JobResponse get_folder_job(folder_id, job_id)
Get status of a folder job in GDS

Get status of a folder job in GDS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Unique identifier for the folder related to the job. | [required] |
**job_id** | **String** | The id of the job | [required] |

### Return type

[**crate::models::JobResponse**](JobResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folder_session

> crate::models::SessionResponse get_folder_session(folder_id, session_id)
Get status of a folder upload in GDS

Get status of a folder upload in GDS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Unique identifier for the folder related to the upload session. | [required] |
**session_id** | **String** | The id of the upload session | [required] |

### Return type

[**crate::models::SessionResponse**](SessionResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_folders

> crate::models::FolderListResponse list_folders(volume_id, volume_name, path, job_statuses, acls, recursive, page_size, page_token, include, tenant_id, metadata_include, metadata_exclude)
Get a list of folders

Given a volumeId or volume name, get a list of folders accessible by the JWT. The default sort returned is alphabetical, ascending. The default page size is 10 items

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | Option<[**Vec<String>**](String.md)> | Optional field that specifies comma-separated volume IDs to include in the list |  |
**volume_name** | Option<[**Vec<String>**](String.md)> | Optional field that specifies comma-separated volume names to include in the list |  |
**path** | Option<[**Vec<String>**](String.md)> | Optional field that specifies comma-separated paths to include in the list. Value can use wildcards (e.g. /a/b/c/_*) or exact matches (e.g. /a/b/c/d/). |  |
**job_statuses** | Option<**String**> | Optional field that specifies comma-separated JobStatuses to include in the list |  |
**acls** | Option<[**Vec<String>**](String.md)> | Optional field that specifies comma-separated acls to include in the list |  |
**recursive** | Option<**bool**> | Optional field to specify if folders should be returned recursively in and under the specified paths, or only directly in the specified paths |  |
**page_size** | Option<**i32**> | START_DESC END_DESC |  |
**page_token** | Option<**String**> | START_DESC END_DESC |  |
**include** | Option<**String**> | Optionally include additional fields in the response. Multiple fields can be included by comma-separation.  Possible values: TotalItemCount, InheritedAcl |  |
**tenant_id** | Option<**String**> | Optional parameter to see shared data in another tenant |  |
**metadata_include** | Option<**String**> | Optional parameter to specify comma separated patterns to include metadata by their field names. |  |
**metadata_exclude** | Option<**String**> | Optional parameter to specify comma separated patterns to exclude metadata by their field names. |  |

### Return type

[**crate::models::FolderListResponse**](FolderListResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unarchive_folder

> crate::models::FolderResponse unarchive_folder(folder_id, body)
Unarchive a folder

Unarchive a folder from a lower storage cost tier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Unique identifier for the folder to be unarchived. | [required] |
**body** | [**FolderUnarchiveRequest**](FolderUnarchiveRequest.md) |  | [required] |

### Return type

[**crate::models::FolderResponse**](FolderResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_folder

> crate::models::FolderWriteableResponse update_folder(folder_id, include, body)
Update a folder content or acl

Update an existing folder in GDS and return upload credentials for that folder. Changes to the folder name and other metadata are not supported at this time.  Optionally overwrite the acl for this folder if acl is provided in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Unique identifier for the folder to be updated. | [required] |
**include** | Option<**String**> | Optionally include additional fields in the response.              Possible values: ObjectStoreAccess |  |
**body** | Option<[**FolderUpdateRequest**](FolderUpdateRequest.md)> |  |  |

### Return type

[**crate::models::FolderWriteableResponse**](FolderWriteableResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

