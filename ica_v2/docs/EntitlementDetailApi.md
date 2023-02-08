# \EntitlementDetailApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_all_matching_activation_codes_for_cwl**](EntitlementDetailApi.md#find_all_matching_activation_codes_for_cwl) | **POST** /api/activationCodes:findAllMatchingForCwl | Search all matching activation code details for a Cwl pipeline.
[**find_all_matching_activation_codes_for_nextflow**](EntitlementDetailApi.md#find_all_matching_activation_codes_for_nextflow) | **POST** /api/activationCodes:findAllMatchingForNextflow | Search all matching activation code details for a Nextflow pipeline.
[**find_best_matching_activation_code_for_cwl**](EntitlementDetailApi.md#find_best_matching_activation_code_for_cwl) | **POST** /api/activationCodes:findBestMatchingForCwl | Search the best matching activation code detail for Cwl pipeline.
[**find_best_matching_activation_codes_for_nextflow**](EntitlementDetailApi.md#find_best_matching_activation_codes_for_nextflow) | **POST** /api/activationCodes:findBestMatchingForNextflow | Search the best matching activation code details for Nextflow pipeline.



## find_all_matching_activation_codes_for_cwl

> crate::models::ActivationCodeDetailList find_all_matching_activation_codes_for_cwl(search_matching_activation_codes_for_cwl_analysis)
Search all matching activation code details for a Cwl pipeline.

Endpoint for searching all matching activation code details for a project and an analysis from a Cwl pipeline.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_matching_activation_codes_for_cwl_analysis** | Option<[**SearchMatchingActivationCodesForCwlAnalysis**](SearchMatchingActivationCodesForCwlAnalysis.md)> |  |  |

### Return type

[**crate::models::ActivationCodeDetailList**](ActivationCodeDetailList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_matching_activation_codes_for_nextflow

> crate::models::ActivationCodeDetailList find_all_matching_activation_codes_for_nextflow(search_matching_activation_codes_for_nextflow_analysis)
Search all matching activation code details for a Nextflow pipeline.

Endpoint for searching all matching activation code details for a project and an analysis from a Nextflow pipeline.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_matching_activation_codes_for_nextflow_analysis** | Option<[**SearchMatchingActivationCodesForNextflowAnalysis**](SearchMatchingActivationCodesForNextflowAnalysis.md)> |  |  |

### Return type

[**crate::models::ActivationCodeDetailList**](ActivationCodeDetailList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_best_matching_activation_code_for_cwl

> crate::models::ActivationCodeDetail find_best_matching_activation_code_for_cwl(search_matching_activation_codes_for_cwl_analysis)
Search the best matching activation code detail for Cwl pipeline.

Endpoint for searching the best activation code detail for a project and an analysis from a Cwl pipeline.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_matching_activation_codes_for_cwl_analysis** | Option<[**SearchMatchingActivationCodesForCwlAnalysis**](SearchMatchingActivationCodesForCwlAnalysis.md)> |  |  |

### Return type

[**crate::models::ActivationCodeDetail**](ActivationCodeDetail.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_best_matching_activation_codes_for_nextflow

> crate::models::ActivationCodeDetail find_best_matching_activation_codes_for_nextflow(search_matching_activation_codes_for_nextflow_analysis)
Search the best matching activation code details for Nextflow pipeline.

Endpoint for searching the best activation code details for a project and an analysis for a Nextflow pipeline.This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_matching_activation_codes_for_nextflow_analysis** | Option<[**SearchMatchingActivationCodesForNextflowAnalysis**](SearchMatchingActivationCodesForNextflowAnalysis.md)> |  |  |

### Return type

[**crate::models::ActivationCodeDetail**](ActivationCodeDetail.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

