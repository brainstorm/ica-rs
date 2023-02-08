# \StorageCredentialsApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_storage_credential**](StorageCredentialsApi.md#create_storage_credential) | **POST** /api/storageCredentials | Create a new storage credential
[**get_storage_credential**](StorageCredentialsApi.md#get_storage_credential) | **GET** /api/storageCredentials/{storageCredentialId} | Retrieve a storage credential.
[**get_storage_credentials**](StorageCredentialsApi.md#get_storage_credentials) | **GET** /api/storageCredentials | Retrieve a list of storage credentials.
[**share_storage_credential**](StorageCredentialsApi.md#share_storage_credential) | **POST** /api/storageCredentials/{storageCredentialId}:share | Share your own storage credentials with tenant.
[**update_storage_credential_secrets**](StorageCredentialsApi.md#update_storage_credential_secrets) | **POST** /api/storageCredentials/{storageCredentialId}:updateSecrets | Update a storage credential's secrets.



## create_storage_credential

> crate::models::StorageCredential create_storage_credential(create_storage_credential)
Create a new storage credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_storage_credential** | Option<[**CreateStorageCredential**](CreateStorageCredential.md)> |  |  |

### Return type

[**crate::models::StorageCredential**](StorageCredential.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_credential

> crate::models::StorageCredential get_storage_credential(storage_credential_id)
Retrieve a storage credential.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_credential_id** | **String** | The ID of the storage credential to retrieve | [required] |

### Return type

[**crate::models::StorageCredential**](StorageCredential.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_credentials

> crate::models::StorageCredentialList get_storage_credentials()
Retrieve a list of storage credentials.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StorageCredentialList**](StorageCredentialList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_storage_credential

> share_storage_credential(storage_credential_id)
Share your own storage credentials with tenant.

Here you share your own storage credentials with all the other users in your tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_credential_id** | **String** | The ID of the storage credential to share | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_storage_credential_secrets

> update_storage_credential_secrets(storage_credential_id, update_storage_credential_secrets)
Update a storage credential's secrets.

When your storage credentials change or get updated due to security reasons you need to update them here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_credential_id** | **String** |  | [required] |
**update_storage_credential_secrets** | Option<[**UpdateStorageCredentialSecrets**](UpdateStorageCredentialSecrets.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

