# \ProjectCustomNotificationSubscriptionsApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_notification_subscription**](ProjectCustomNotificationSubscriptionsApi.md#create_notification_subscription) | **POST** /api/projects/{projectId}/customNotificationSubscriptions | Create a custom notification subscription
[**delete_notification_subscription**](ProjectCustomNotificationSubscriptionsApi.md#delete_notification_subscription) | **DELETE** /api/projects/{projectId}/customNotificationSubscriptions/{subscriptionId} | Delete a custom notification subscription
[**get_notification_subscription**](ProjectCustomNotificationSubscriptionsApi.md#get_notification_subscription) | **GET** /api/projects/{projectId}/customNotificationSubscriptions/{subscriptionId} | Retrieve a notification subscription
[**get_notification_subscriptions**](ProjectCustomNotificationSubscriptionsApi.md#get_notification_subscriptions) | **GET** /api/projects/{projectId}/customNotificationSubscriptions | Retrieve notification subscriptions
[**update_notification_subscription**](ProjectCustomNotificationSubscriptionsApi.md#update_notification_subscription) | **PUT** /api/projects/{projectId}/customNotificationSubscriptions/{subscriptionId} | Update a notification subscription



## create_notification_subscription

> crate::models::CustomNotificationSubscription create_notification_subscription(project_id, create_custom_notification_subscription)
Create a custom notification subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**create_custom_notification_subscription** | Option<[**CreateCustomNotificationSubscription**](CreateCustomNotificationSubscription.md)> | The new subscription |  |

### Return type

[**crate::models::CustomNotificationSubscription**](CustomNotificationSubscription.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification_subscription

> delete_notification_subscription(project_id, subscription_id)
Delete a custom notification subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**subscription_id** | **String** | The ID of the custom notification subscription to delete | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_subscription

> crate::models::CustomNotificationSubscription get_notification_subscription(project_id, subscription_id)
Retrieve a notification subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project | [required] |
**subscription_id** | **String** | The ID of the notification subscription | [required] |

### Return type

[**crate::models::CustomNotificationSubscription**](CustomNotificationSubscription.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_subscriptions

> crate::models::CustomNotificationSubscriptionList get_notification_subscriptions(project_id)
Retrieve notification subscriptions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project | [required] |

### Return type

[**crate::models::CustomNotificationSubscriptionList**](CustomNotificationSubscriptionList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notification_subscription

> crate::models::CustomNotificationSubscription update_notification_subscription(project_id, subscription_id, if_match, custom_notification_subscription)
Update a notification subscription

Fields which can be updated:  - enabled  - eventCode  - filterExpression  - notificationChannel 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project | [required] |
**subscription_id** | **String** | The ID of the custom notification subscription to update | [required] |
**if_match** | Option<**String**> | Optional header parameter to enable conflict exposure. If the client provides this header, then it must contains the client's most recent value of the 'ETag' response header, and the server will respond with a 409 code if it detects a conflict. If the client does not provide this header, then the server will not do a conflict check, which means that as a client you can override the resource even when the server has a more recent version. |  |
**custom_notification_subscription** | Option<[**CustomNotificationSubscription**](CustomNotificationSubscription.md)> | The updated subscription |  |

### Return type

[**crate::models::CustomNotificationSubscription**](CustomNotificationSubscription.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

