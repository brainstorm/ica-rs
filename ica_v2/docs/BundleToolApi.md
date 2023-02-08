# \BundleToolApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bundle_tools**](BundleToolApi.md#get_bundle_tools) | **GET** /api/bundles/{bundleId}/tools | Retrieve a list of bundle tools.
[**get_tools_eligible_for_linking_to_bundle**](BundleToolApi.md#get_tools_eligible_for_linking_to_bundle) | **GET** /api/bundles/{bundleId}/tools/eligibleForLinking | Retrieve a list of tools eligible for linking to the bundle.
[**link_tool_to_bundle**](BundleToolApi.md#link_tool_to_bundle) | **POST** /api/bundles/{bundleId}/tools/{toolId} | Link a tool to a bundle
[**unlink_tool_from_bundle**](BundleToolApi.md#unlink_tool_from_bundle) | **DELETE** /api/bundles/{bundleId}/tools/{toolId} | Unlink a tool from this bundle.



## get_bundle_tools

> crate::models::BundleToolsList get_bundle_tools(bundle_id)
Retrieve a list of bundle tools.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** | The ID of the bundle to get tools from | [required] |

### Return type

[**crate::models::BundleToolsList**](BundleToolsList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tools_eligible_for_linking_to_bundle

> crate::models::CwlToolDefinitionList get_tools_eligible_for_linking_to_bundle(bundle_id)
Retrieve a list of tools eligible for linking to the bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** | The ID of the bundle to get the eligible tools for | [required] |

### Return type

[**crate::models::CwlToolDefinitionList**](CwlToolDefinitionList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_tool_to_bundle

> link_tool_to_bundle(bundle_id, tool_id)
Link a tool to a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** | The ID of the bundle to link the tool to | [required] |
**tool_id** | **String** | The ID of the tool to link | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_tool_from_bundle

> unlink_tool_from_bundle(bundle_id, tool_id)
Unlink a tool from this bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_id** | **String** |  | [required] |
**tool_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

