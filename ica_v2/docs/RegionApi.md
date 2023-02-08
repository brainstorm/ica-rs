# \RegionApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_region**](RegionApi.md#get_region) | **GET** /api/regions/{regionId} | Retrieve a region. Only the regions the user has access to through his/her entitlements can be retrieved.
[**get_regions**](RegionApi.md#get_regions) | **GET** /api/regions | Retrieve a list of regions. Only the regions the user has access to through his/her entitlements are returned.



## get_region

> crate::models::Region get_region(region_id)
Retrieve a region. Only the regions the user has access to through his/her entitlements can be retrieved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region_id** | **String** |  | [required] |

### Return type

[**crate::models::Region**](Region.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_regions

> crate::models::RegionList get_regions()
Retrieve a list of regions. Only the regions the user has access to through his/her entitlements are returned.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RegionList**](RegionList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

