# \FilesApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_file**](FilesApi.md#archive_file) | **POST** /v1/files/{fileId}:archive | Archive a file
[**bulk_file_update**](FilesApi.md#bulk_file_update) | **PATCH** /v1/files | Updates list of files with metadata
[**complete_file_upload**](FilesApi.md#complete_file_upload) | **POST** /v1/files/{fileId}:completeUpload | Complete a file Upload
[**create_file**](FilesApi.md#create_file) | **POST** /v1/files | Create a file entry in GDS and get temporary credentials for upload
[**delete_file**](FilesApi.md#delete_file) | **DELETE** /v1/files/{fileId} | Permanently delete a file
[**get_file**](FilesApi.md#get_file) | **GET** /v1/files/{fileId} | Get details about a file, including a pre-signed URL for download
[**list_files**](FilesApi.md#list_files) | **GET** /v1/files | Get a list of files
[**list_volume_files**](FilesApi.md#list_volume_files) | **POST** /v1/files/list | Get a list of volume files
[**unarchive_file**](FilesApi.md#unarchive_file) | **POST** /v1/files/{fileId}:unarchive | Unarchive a file
[**update_file**](FilesApi.md#update_file) | **PATCH** /v1/files/{fileId} | Update a file entry in GDS and get temporary credentials for upload



## archive_file

> crate::models::FileResponse archive_file(file_id, body)
Archive a file

Archives a file to a lower storage cost tier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Unique identifier for the file to be archived. | [required] |
**body** | [**FileArchiveRequest**](FileArchiveRequest.md) |  | [required] |

### Return type

[**crate::models::FileResponse**](FileResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_file_update

> crate::models::BulkFileUpdateResponse bulk_file_update(body)
Updates list of files with metadata

Updates list of files with metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**BulkFileUpdateRequest**](BulkFileUpdateRequest.md)> |  |  |

### Return type

[**crate::models::BulkFileUpdateResponse**](BulkFileUpdateResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_file_upload

> crate::models::FileResponse complete_file_upload(file_id, body)
Complete a file Upload

Complete a file upload operation. If the file was uploaded using multipart uploads, combine all the multiple parts uploaded into one complete file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Unique identifier for the file upload to be completed. | [required] |
**body** | [**FileUploadCompleteRequest**](FileUploadCompleteRequest.md) |  | [required] |

### Return type

[**crate::models::FileResponse**](FileResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_file

> crate::models::FileWriteableResponse create_file(body, include, upload_part_count)
Create a file entry in GDS and get temporary credentials for upload

Create a file entry in GDS. Returns temporary credentials and presigned url(s) for file upload directly to S3 when the include=objectStoreAccess parameter is used. Volume ID or volume name is required for file creation. If a folder path is provided and does not exist, GDS creates the folder path in the appropriate account automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateFileRequest**](CreateFileRequest.md) |  | [required] |
**include** | Option<**String**> | Optionally include additional fields in the response.              Possible values: ObjectStoreAccess |  |
**upload_part_count** | Option<**i32**> | Optional number of parts for the presigned url for uploads (1 - 10000) |  |

### Return type

[**crate::models::FileWriteableResponse**](FileWriteableResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file

> delete_file(file_id)
Permanently delete a file

Permanently delete a file entry and its underlying content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Unique identifier for the file to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file

> crate::models::FileResponse get_file(file_id, tenant_id, presigned_url_mode, include_volume_metadata, metadata_include, metadata_exclude)
Get details about a file, including a pre-signed URL for download

Get information and details for the specified file ID, including metadata and a pre-signed URL for file download. The URL can be used as a curl command or directly with S3.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Unique identifier for the file to retrieve. | [required] |
**tenant_id** | Option<**String**> | Optional parameter to see shared data in another tenant |  |
**presigned_url_mode** | Option<**String**> | Optional parameter to specify presigned url's content-disposition. If not specified, the browser will determine the default behavior.              Possible values: Attachment, Inline, Browser |  |
**include_volume_metadata** | Option<**bool**> | Optional parameter to return volume's metadata |  |
**metadata_include** | Option<**String**> | Optional parameter to specify comma separated patterns to include metadata by their field names. |  |
**metadata_exclude** | Option<**String**> | Optional parameter to specify comma separated patterns to exclude metadata by their field names. |  |

### Return type

[**crate::models::FileResponse**](FileResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_files

> crate::models::FileListResponse list_files(volume_id, volume_name, path, is_uploaded, archive_status, recursive, presigned_url_mode, include, page_size, page_token, tenant_id, metadata_include, metadata_exclude)
Get a list of files

Given a volumeId or volume name, get a list of files accessible by the JWT. The default sort returned is alphabetical, ascending. The default page size is 10 items

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | Option<[**Vec<String>**](String.md)> | Optional field that specifies comma-separated volume IDs to include in the list |  |
**volume_name** | Option<[**Vec<String>**](String.md)> | Optional field that specifies comma-separated volume names to include in the list |  |
**path** | Option<[**Vec<String>**](String.md)> | Optional field that specifies comma-separated paths to include in the list. Value can use wildcards (e.g. /a/b/c/_*) or exact matches (e.g. /a/b/c/d/). |  |
**is_uploaded** | Option<**bool**> | Optional field to filter by Uploaded files |  |
**archive_status** | Option<**String**> | Optional field that specifies comma-separated Archive Statuses to include in the list |  |
**recursive** | Option<**bool**> | Optional field to specify if files should be returned recursively in and under the specified paths, or only directly in the specified paths |  |
**presigned_url_mode** | Option<**String**> | Optional parameter to specify presigned url's content-disposition. If not specified, the browser will determine the default behavior.  Possible values: Attachment, Inline, Browser |  |
**include** | Option<**String**> | Optionally include additional fields in the response. Multiple fields can be included by comma-separation.  Possible values: TotalItemCount, PresignedUrl, InheritedAcl |  |
**page_size** | Option<**i32**> | START_DESC END_DESC |  |
**page_token** | Option<**String**> | START_DESC END_DESC |  |
**tenant_id** | Option<**String**> | Optional parameter to see shared data in another tenant |  |
**metadata_include** | Option<**String**> | Optional parameter to specify comma separated patterns to include metadata by their field names. |  |
**metadata_exclude** | Option<**String**> | Optional parameter to specify comma separated patterns to exclude metadata by their field names. |  |

### Return type

[**crate::models::FileListResponse**](FileListResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_volume_files

> crate::models::VolumeFileListResponse list_volume_files(body)
Get a list of volume files

Gets file list by volume ID and an array of file IDs. The default sort returned is alphabetical, ascending

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**VolumeFileListRequest**](VolumeFileListRequest.md) |  | [required] |

### Return type

[**crate::models::VolumeFileListResponse**](VolumeFileListResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unarchive_file

> crate::models::FileResponse unarchive_file(file_id, body)
Unarchive a file

Unarchive a file from a lower storage cost tier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Unique identifier for the file to be unarchived. | [required] |
**body** | [**FileUnarchiveRequest**](FileUnarchiveRequest.md) |  | [required] |

### Return type

[**crate::models::FileResponse**](FileResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_file

> crate::models::FileWriteableResponse update_file(file_id, include, upload_part_count, body)
Update a file entry in GDS and get temporary credentials for upload

Update a file entry in GDS. Returns temporary credentials and presigned url(s) for file upload directly to S3 when the include=objectStoreAccess parameter is used. Note that the currently supported changes to the file resource are updating the file type and the underlying content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | Unique identifier for the file to be updated. | [required] |
**include** | Option<**String**> | Optionally include additional fields in the response.              Possible values: ObjectStoreAccess |  |
**upload_part_count** | Option<**i32**> | Optional number of parts for the presigned url for uploads (1 - 10000) |  |
**body** | Option<[**UpdateFileRequest**](UpdateFileRequest.md)> |  |  |

### Return type

[**crate::models::FileWriteableResponse**](FileWriteableResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

