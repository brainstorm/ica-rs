# \StorageConfigurationApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_storage_configuration**](StorageConfigurationApi.md#create_storage_configuration) | **POST** /api/storageConfigurations | Create a new storage configuration
[**get_storage_configuration**](StorageConfigurationApi.md#get_storage_configuration) | **GET** /api/storageConfigurations/{storageConfigurationId} | Retrieve a storage configuration.
[**get_storage_configuration_details**](StorageConfigurationApi.md#get_storage_configuration_details) | **GET** /api/storageConfigurations/{storageConfigurationId}/details | Retrieve a storage configuration detail.
[**get_storage_configurations**](StorageConfigurationApi.md#get_storage_configurations) | **GET** /api/storageConfigurations | Retrieve a list of storage configurations.
[**share_storage_configuration**](StorageConfigurationApi.md#share_storage_configuration) | **POST** /api/storageConfigurations/{storageConfigurationId}:share | Share your own storage configuration with tenant.



## create_storage_configuration

> crate::models::StorageConfiguration create_storage_configuration(create_storage_configuration)
Create a new storage configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_storage_configuration** | Option<[**CreateStorageConfiguration**](CreateStorageConfiguration.md)> |  |  |

### Return type

[**crate::models::StorageConfiguration**](StorageConfiguration.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_configuration

> crate::models::StorageConfiguration get_storage_configuration(storage_configuration_id)
Retrieve a storage configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_configuration_id** | **String** | The ID of the storage configuration to retrieve | [required] |

### Return type

[**crate::models::StorageConfiguration**](StorageConfiguration.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_configuration_details

> crate::models::StorageConfigurationDetails get_storage_configuration_details(storage_configuration_id)
Retrieve a storage configuration detail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_configuration_id** | **String** | The ID of the storage configuration to retrieve | [required] |

### Return type

[**crate::models::StorageConfigurationDetails**](StorageConfigurationDetails.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_configurations

> crate::models::StorageConfigurationWithDetailsList get_storage_configurations()
Retrieve a list of storage configurations.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StorageConfigurationWithDetailsList**](StorageConfigurationWithDetailsList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_storage_configuration

> share_storage_configuration(storage_configuration_id)
Share your own storage configuration with tenant.

Here you share your own storage configuration with all the other users in your tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_configuration_id** | **String** | The ID of the storage configuration to share | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

