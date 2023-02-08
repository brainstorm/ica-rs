# \ReferenceSetApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_reference_sets**](ReferenceSetApi.md#get_reference_sets) | **GET** /api/referenceSets | Retrieve a list of of reference sets.
[**get_species**](ReferenceSetApi.md#get_species) | **GET** /api/referenceSets/{referenceSetId}/species | Retrieve a list of species linked to the reference set.



## get_reference_sets

> crate::models::ReferenceSetList get_reference_sets()
Retrieve a list of of reference sets.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ReferenceSetList**](ReferenceSetList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_species

> crate::models::SpeciesList get_species(reference_set_id)
Retrieve a list of species linked to the reference set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reference_set_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::SpeciesList**](SpeciesList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

