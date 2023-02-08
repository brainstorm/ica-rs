# \MetadataModelApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_metadata_model**](MetadataModelApi.md#get_metadata_model) | **GET** /api/metadataModels/{metadataModelId} | Retrieve a metadata model. Only metadata models that the user has access to can be retrieved.
[**get_metadata_model_fields**](MetadataModelApi.md#get_metadata_model_fields) | **GET** /api/metadataModels/{metadataModelId}/fields | Retrieve the fields of a metadata model. Only metadata models that the user has access to can be retrieved.
[**get_metadata_models**](MetadataModelApi.md#get_metadata_models) | **GET** /api/metadataModels | Retrieve the metadata models for the tenant associated to the security context.
[**get_tenant_model**](MetadataModelApi.md#get_tenant_model) | **GET** /api/metadataModels/tenantModel | Retrieve the tenant model for the tenant associated to the security context.



## get_metadata_model

> crate::models::MetadataModel get_metadata_model(metadata_model_id)
Retrieve a metadata model. Only metadata models that the user has access to can be retrieved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata_model_id** | **String** |  | [required] |

### Return type

[**crate::models::MetadataModel**](MetadataModel.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_model_fields

> crate::models::FieldList get_metadata_model_fields(metadata_model_id)
Retrieve the fields of a metadata model. Only metadata models that the user has access to can be retrieved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata_model_id** | **String** |  | [required] |

### Return type

[**crate::models::FieldList**](FieldList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_models

> crate::models::MetadataModelList get_metadata_models()
Retrieve the metadata models for the tenant associated to the security context.

Retrieve the metadata models for the tenant associated to the security context. This call returns a list of metadata models for the tenant in a non-hierarchical way. Instead of a model having a list of child models all models except the root model have a parent model identifier. This can be used to reconstruct the hierarchy.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MetadataModelList**](MetadataModelList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_model

> crate::models::Model get_tenant_model()
Retrieve the tenant model for the tenant associated to the security context.

Retrieve the tenant model for the tenant associated to the security context. The tenant model is a hierarchical structure where the top level tenant holds a list of child models (which in turn can hold child models).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Model**](Model.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

