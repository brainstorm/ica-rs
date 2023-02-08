# \VolumeConfigurationsApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_volume_configuration**](VolumeConfigurationsApi.md#create_volume_configuration) | **POST** /v1/volumeconfigurations | Create a volume configuration in GDS.
[**delete_volume_configuration**](VolumeConfigurationsApi.md#delete_volume_configuration) | **DELETE** /v1/volumeconfigurations/{volumeConfigurationName} | Deletes a volume configuration by Id or name
[**get_volume_configuration**](VolumeConfigurationsApi.md#get_volume_configuration) | **GET** /v1/volumeconfigurations/{volumeConfigurationName} | Get information for the specified volume configuration name or Id
[**list_volume_configurations**](VolumeConfigurationsApi.md#list_volume_configurations) | **GET** /v1/volumeconfigurations | Get a list of volumes
[**validate_volume_configuration**](VolumeConfigurationsApi.md#validate_volume_configuration) | **POST** /v1/volumeconfigurations/{volumeConfigurationName}:validate | Validate a volume configuration



## create_volume_configuration

> crate::models::VolumeConfigurationResponse create_volume_configuration(body)
Create a volume configuration in GDS.

Create a volume configuration in GDS. This contains the object store details that will be used to create volumes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateVolumeConfigurationRequest**](CreateVolumeConfigurationRequest.md) |  | [required] |

### Return type

[**crate::models::VolumeConfigurationResponse**](VolumeConfigurationResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_volume_configuration

> crate::models::VolumeResponse delete_volume_configuration(volume_configuration_name)
Deletes a volume configuration by Id or name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_configuration_name** | **String** | Unique name of the Volume Configuration to be deleted. | [required] |

### Return type

[**crate::models::VolumeResponse**](VolumeResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume_configuration

> crate::models::VolumeConfigurationResponse get_volume_configuration(volume_configuration_name)
Get information for the specified volume configuration name or Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_configuration_name** | **String** | Unique name of the volume configuration to retrieve information for. | [required] |

### Return type

[**crate::models::VolumeConfigurationResponse**](VolumeConfigurationResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_volume_configurations

> crate::models::VolumeConfigurationListResponse list_volume_configurations(online_status, page_size, page_token, include)
Get a list of volumes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**online_status** | Option<**String**> | Optional field that specifies the Online Status for Volume configurations included in the list.  If provided, the value must be Initializing, Ok, or Error. |  |
**page_size** | Option<**i32**> | START_DESC END_DESC |  |
**page_token** | Option<**String**> | START_DESC END_DESC |  |
**include** | Option<**String**> | Optionally include additional fields in the response. Multiple fields can be included by comma-separation.  Possible values: TotalItemCount, InheritedAcl |  |

### Return type

[**crate::models::VolumeConfigurationListResponse**](VolumeConfigurationListResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_volume_configuration

> crate::models::VolumeConfigurationResponse validate_volume_configuration(volume_configuration_name)
Validate a volume configuration

Validate an existing volume configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_configuration_name** | **String** | Unique name of the volume configuration to be validated. | [required] |

### Return type

[**crate::models::VolumeConfigurationResponse**](VolumeConfigurationResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

