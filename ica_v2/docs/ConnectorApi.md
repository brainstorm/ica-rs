# \ConnectorApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_connector**](ConnectorApi.md#cancel_connector) | **POST** /api/connectors/{connectorId}:cancel | Cancel a connector.
[**create_connector**](ConnectorApi.md#create_connector) | **POST** /api/connectors | Create a connector.
[**create_download_rule**](ConnectorApi.md#create_download_rule) | **POST** /api/connectors/{connectorId}/downloadRules | Create a download rule.
[**create_upload_rule**](ConnectorApi.md#create_upload_rule) | **POST** /api/connectors/{connectorId}/uploadRules | Create an upload rule.
[**delete_download_rule**](ConnectorApi.md#delete_download_rule) | **DELETE** /api/connectors/{connectorId}/downloadRules/{downloadRuleId} | Delete a download rule.
[**delete_upload_rule**](ConnectorApi.md#delete_upload_rule) | **DELETE** /api/connectors/{connectorId}/uploadRules/{uploadRuleId} | Delete an upload rule.
[**disable_connector**](ConnectorApi.md#disable_connector) | **POST** /api/connectors/{connectorId}:disable | Disable a connector.
[**enable_connector**](ConnectorApi.md#enable_connector) | **POST** /api/connectors/{connectorId}:enable | Enable a connector.
[**get_connector**](ConnectorApi.md#get_connector) | **GET** /api/connectors/{connectorId} | Retrieve a connector.
[**get_connectors**](ConnectorApi.md#get_connectors) | **GET** /api/connectors | Retrieve a list of connectors.
[**get_download_rule**](ConnectorApi.md#get_download_rule) | **GET** /api/connectors/{connectorId}/downloadRules/{downloadRuleId} | Retrieve a download rule.
[**get_download_rules**](ConnectorApi.md#get_download_rules) | **GET** /api/connectors/{connectorId}/downloadRules | Retrieve a list of download rules.
[**get_upload_rule**](ConnectorApi.md#get_upload_rule) | **GET** /api/connectors/{connectorId}/uploadRules/{uploadRuleId} | Retrieve an upload rule.
[**get_upload_rules**](ConnectorApi.md#get_upload_rules) | **GET** /api/connectors/{connectorId}/uploadRules | Retrieve a list of upload rules.
[**update_download_rule**](ConnectorApi.md#update_download_rule) | **PUT** /api/connectors/{connectorId}/downloadRules/{downloadRuleId} | Update a download rule.
[**update_upload_rule**](ConnectorApi.md#update_upload_rule) | **PUT** /api/connectors/{connectorId}/uploadRules/{uploadRuleId} | Update an upload rule.



## cancel_connector

> cancel_connector(connector_id)
Cancel a connector.

Endpoint for cancelling a connector. This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_connector

> crate::models::Connector create_connector(create_connector)
Create a connector.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_connector** | Option<[**CreateConnector**](CreateConnector.md)> | The connector to create. |  |

### Return type

[**crate::models::Connector**](Connector.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_download_rule

> crate::models::DownloadRule create_download_rule(connector_id, create_download_rule)
Create a download rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |
**create_download_rule** | Option<[**CreateDownloadRule**](CreateDownloadRule.md)> |  |  |

### Return type

[**crate::models::DownloadRule**](DownloadRule.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_upload_rule

> crate::models::UploadRule create_upload_rule(connector_id, create_upload_rule)
Create an upload rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |
**create_upload_rule** | Option<[**CreateUploadRule**](CreateUploadRule.md)> |  |  |

### Return type

[**crate::models::UploadRule**](UploadRule.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_download_rule

> delete_download_rule(connector_id, download_rule_id)
Delete a download rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |
**download_rule_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_upload_rule

> delete_upload_rule(connector_id, upload_rule_id)
Delete an upload rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |
**upload_rule_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_connector

> disable_connector(connector_id)
Disable a connector.

Endpoint for disabling a connector. This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_connector

> enable_connector(connector_id)
Enable a connector.

Endpoint for enabling a connector. This is a non-RESTful endpoint, as the path of this endpoint is not representing a REST resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connector

> crate::models::Connector get_connector(connector_id)
Retrieve a connector.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |

### Return type

[**crate::models::Connector**](Connector.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connectors

> crate::models::ConnectorList get_connectors(active_only)
Retrieve a list of connectors.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**active_only** | Option<**bool**> | When true only the active connectors will be returned. When false (default value) all connectors wil be returned. |  |

### Return type

[**crate::models::ConnectorList**](ConnectorList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_download_rule

> crate::models::DownloadRule get_download_rule(connector_id, download_rule_id)
Retrieve a download rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |
**download_rule_id** | **String** |  | [required] |

### Return type

[**crate::models::DownloadRule**](DownloadRule.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_download_rules

> crate::models::DownloadRuleList get_download_rules(connector_id)
Retrieve a list of download rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |

### Return type

[**crate::models::DownloadRuleList**](DownloadRuleList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_upload_rule

> crate::models::UploadRule get_upload_rule(connector_id, upload_rule_id)
Retrieve an upload rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |
**upload_rule_id** | **String** |  | [required] |

### Return type

[**crate::models::UploadRule**](UploadRule.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_upload_rules

> crate::models::UploadRuleList get_upload_rules(connector_id)
Retrieve a list of upload rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |

### Return type

[**crate::models::UploadRuleList**](UploadRuleList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_download_rule

> crate::models::DownloadRule update_download_rule(connector_id, download_rule_id, if_match, download_rule)
Update a download rule.

Fields which can be updated:  - code  - active  - description  - sequence  - formatCode  - projectName  - targetLocalFolder  - protocol  - fileNameExpression  - disableHashing

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |
**download_rule_id** | **String** |  | [required] |
**if_match** | Option<**String**> | Optional header parameter to enable conflict exposure. If the client provides this header, then it must contains the client's most recent value of the 'ETag' response header, and the server will respond with a 409 code if it detects a conflict. If the client does not provide this header, then the server will not do a conflict check, which means that as a client you can override the resource even when the server has a more recent version. |  |
**download_rule** | Option<[**DownloadRule**](DownloadRule.md)> |  |  |

### Return type

[**crate::models::DownloadRule**](DownloadRule.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_upload_rule

> crate::models::UploadRule update_upload_rule(connector_id, upload_rule_id, if_match, upload_rule)
Update an upload rule.

Fields which can be updated:  - code  - active  - description  - localFolder  - filePattern  - dataFormat 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** |  | [required] |
**upload_rule_id** | **String** |  | [required] |
**if_match** | Option<**String**> | Optional header parameter to enable conflict exposure. If the client provides this header, then it must contains the client's most recent value of the 'ETag' response header, and the server will respond with a 409 code if it detects a conflict. If the client does not provide this header, then the server will not do a conflict check, which means that as a client you can override the resource even when the server has a more recent version. |  |
**upload_rule** | Option<[**UploadRule**](UploadRule.md)> |  |  |

### Return type

[**crate::models::UploadRule**](UploadRule.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

