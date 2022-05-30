# \VolumesApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_volume**](VolumesApi.md#create_volume) | **POST** /v1/volumes | Create a volume in GDS and receive temporary credentials for upload
[**delete_volume**](VolumesApi.md#delete_volume) | **DELETE** /v1/volumes/{volumeId} | Deletes a volume by Id
[**get_volume**](VolumesApi.md#get_volume) | **GET** /v1/volumes/{volumeId} | Get information for the specified volume ID or volume name
[**list_volumes**](VolumesApi.md#list_volumes) | **GET** /v1/volumes | Get a list of volumes
[**update_volume**](VolumesApi.md#update_volume) | **PATCH** /v1/volumes/{volumeId} | Update a volume content



## create_volume

> crate::models::CreateVolumeResponse create_volume(body, include)
Create a volume in GDS and receive temporary credentials for upload

Create a volume in GDS to hold folders and files. Returns upload credentials to the root folder of the volume when the include=objectStoreAccess parameter is used. You must create a volume prior to uploading files or folders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateVolumeRequest**](CreateVolumeRequest.md) |  | [required] |
**include** | Option<**String**> | Optionally include additional fields in the response.              Possible values: ObjectStoreAccess |  |

### Return type

[**crate::models::CreateVolumeResponse**](CreateVolumeResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_volume

> crate::models::VolumeResponse delete_volume(volume_id, purge_object_store_data)
Deletes a volume by Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | Unique identifier for the Volume to be deleted. | [required] |
**purge_object_store_data** | Option<**bool**> | Optional and for BYOB only. If true, the volume's data in object storage will be erased.              This field is ignored for non-BYOB volumes where the object store data is always removed upon deleting the volume. |  |

### Return type

[**crate::models::VolumeResponse**](VolumeResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume

> crate::models::VolumeResponse get_volume(volume_id, tenant_id, metadata_include, metadata_exclude)
Get information for the specified volume ID or volume name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | Unique identifier for the volume to retrieve information for. | [required] |
**tenant_id** | Option<**String**> | Optional parameter to see shared data in another tenant |  |
**metadata_include** | Option<**String**> | Optional parameter to specify comma separated patterns to include metadata by their field names. |  |
**metadata_exclude** | Option<**String**> | Optional parameter to specify comma separated patterns to exclude metadata by their field names. |  |

### Return type

[**crate::models::VolumeResponse**](VolumeResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_volumes

> crate::models::VolumeListResponse list_volumes(page_size, page_token, include, tenant_id, volume_configuration_name)
Get a list of volumes

Get a list of volumes accessible by the current JWT token’s tenant ID in GDS. The default sort returned is alphabetical, ascending. The default page size is 10 items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | START_DESC END_DESC |  |
**page_token** | Option<**String**> | START_DESC END_DESC |  |
**include** | Option<**String**> | Optionally include additional fields in the response. Multiple fields can be included by comma-separation.  Possible values: TotalItemCount, InheritedAcl |  |
**tenant_id** | Option<**String**> | Optional parameter to see shared data in another tenant |  |
**volume_configuration_name** | Option<**String**> | Unique name of the volume configuration |  |

### Return type

[**crate::models::VolumeListResponse**](VolumeListResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_volume

> crate::models::VolumeResponse update_volume(volume_id, include, body)
Update a volume content

Update an existing volume in GDS and return upload credentials for that volume. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | Unique identifier for the volume to be updated. | [required] |
**include** | Option<**String**> | Optionally include additional fields in the response.              Possible values: ObjectStoreAccess |  |
**body** | Option<[**UpdateVolumeRequest**](UpdateVolumeRequest.md)> |  |  |

### Return type

[**crate::models::VolumeResponse**](VolumeResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

