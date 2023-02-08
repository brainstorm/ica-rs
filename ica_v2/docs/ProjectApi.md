# \ProjectApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project**](ProjectApi.md#create_project) | **POST** /api/projects | Create a new project.
[**get_project**](ProjectApi.md#get_project) | **GET** /api/projects/{projectId} | Retrieve a project.
[**get_project_bundle**](ProjectApi.md#get_project_bundle) | **GET** /api/projects/{projectId}/bundles/{bundleId} | Retrieve a project bundle.
[**get_project_bundles**](ProjectApi.md#get_project_bundles) | **GET** /api/projects/{projectId}/bundles | Retrieve project bundles.
[**get_projects**](ProjectApi.md#get_projects) | **GET** /api/projects | Retrieve a list of projects.
[**hide_project**](ProjectApi.md#hide_project) | **POST** /api/projects/{projectId}:hide | Hide a project.
[**link_project_bundle**](ProjectApi.md#link_project_bundle) | **POST** /api/projects/{projectId}/bundles/{bundleId} | Link a bundle to a project.
[**unlink_project_bundle**](ProjectApi.md#unlink_project_bundle) | **DELETE** /api/projects/{projectId}/bundles/{bundleId} | Unlink a bundle from a project.
[**update_project**](ProjectApi.md#update_project) | **PUT** /api/projects/{projectId} | Update a project.



## create_project

> crate::models::Project create_project(create_project)
Create a new project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_project** | Option<[**CreateProject**](CreateProject.md)> |  |  |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project

> crate::models::Project get_project(project_id)
Retrieve a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_bundle

> crate::models::ProjectBundle get_project_bundle(project_id, bundle_id)
Retrieve a project bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**bundle_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::ProjectBundle**](ProjectBundle.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_bundles

> crate::models::ProjectBundleList get_project_bundles(project_id)
Retrieve project bundles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**crate::models::ProjectBundleList**](ProjectBundleList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects

> crate::models::ProjectPagedList get_projects(search, user_tags, technical_tags, include_hidden_projects, region, page_offset, page_token, page_size, sort)
Retrieve a list of projects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search |  |
**user_tags** | Option<[**Vec<String>**](String.md)> | User tags to filter on |  |
**technical_tags** | Option<[**Vec<String>**](String.md)> | Technical tags to filter on |  |
**include_hidden_projects** | Option<**bool**> | Include hidden projects. |  |[default to false]
**region** | Option<**String**> | The ID of the region to filter on. |  |
**page_offset** | Option<**String**> | The amount of rows to skip in the result. Ideally this is a multiple of the size parameter. Offset-based pagination has a result limit of 200K rows and does not guarantee unique results across pages |  |
**page_token** | Option<**String**> | The cursor to get subsequent results. The value to use is returned in the result when using cursor-based pagination. Cursor-based pagination guarantees complete and unique results across all pages. |  |
**page_size** | Option<**String**> | The amount of rows to return. Use in combination with the offset or cursor parameter to get subsequent results. |  |
**sort** | Option<**String**> | Which field to order the results by. The default order is ascending, suffix with ' desc' to sort descending (suffix ' asc' also works for ascending). Multiple values should be separated with commas. An example: \"?sort=dateCreated, lastName desc\"  The attributes for which sorting is supported: - name - shortDescription - information |  |

### Return type

[**crate::models::ProjectPagedList**](ProjectPagedList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hide_project

> hide_project(project_id)
Hide a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_project_bundle

> crate::models::ProjectBundle link_project_bundle(project_id, bundle_id)
Link a bundle to a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**bundle_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::ProjectBundle**](ProjectBundle.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_project_bundle

> unlink_project_bundle(project_id, bundle_id)
Unlink a bundle from a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**bundle_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> crate::models::Project update_project(project_id, if_match, project)
Update a project.

Fields which can be updated: - shortDescription - projectInformation - billingMode - dataSharingEnabled - tags - storageBundle - metaDataModel - analysisPriority

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**if_match** | Option<**String**> | Optional header parameter to enable conflict exposure. If the client provides this header, then it must contains the client's most recent value of the 'ETag' response header, and the server will respond with a 409 code if it detects a conflict. If the client does not provide this header, then the server will not do a conflict check, which means that as a client you can override the resource even when the server has a more recent version. |  |
**project** | Option<[**Project**](Project.md)> |  |  |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

