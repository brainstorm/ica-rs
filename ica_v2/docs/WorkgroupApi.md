# \WorkgroupApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_workgroup**](WorkgroupApi.md#get_workgroup) | **GET** /api/workgroups/{workgroupId} | Retrieve a workgroup.
[**get_workgroups**](WorkgroupApi.md#get_workgroups) | **GET** /api/workgroups | Retrieve a list of workgroups.



## get_workgroup

> crate::models::Workgroup get_workgroup(workgroup_id)
Retrieve a workgroup.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workgroup_id** | **uuid::Uuid** | The ID of the workgroup to retrieve | [required] |

### Return type

[**crate::models::Workgroup**](Workgroup.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workgroups

> crate::models::WorkgroupList get_workgroups()
Retrieve a list of workgroups.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WorkgroupList**](WorkgroupList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

