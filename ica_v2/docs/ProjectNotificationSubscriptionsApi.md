# \ProjectNotificationSubscriptionsApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_notification_subscription1**](ProjectNotificationSubscriptionsApi.md#create_notification_subscription1) | **POST** /api/projects/{projectId}/notificationSubscriptions | Create a notification subscription
[**delete_notification_subscription1**](ProjectNotificationSubscriptionsApi.md#delete_notification_subscription1) | **DELETE** /api/projects/{projectId}/notificationSubscriptions/{subscriptionId} | Delete a notification subscription
[**get_notification_subscription1**](ProjectNotificationSubscriptionsApi.md#get_notification_subscription1) | **GET** /api/projects/{projectId}/notificationSubscriptions/{subscriptionId} | Retrieve a notification subscription
[**get_notification_subscriptions1**](ProjectNotificationSubscriptionsApi.md#get_notification_subscriptions1) | **GET** /api/projects/{projectId}/notificationSubscriptions | Retrieve notification subscriptions
[**update_notification_subscription1**](ProjectNotificationSubscriptionsApi.md#update_notification_subscription1) | **PUT** /api/projects/{projectId}/notificationSubscriptions/{subscriptionId} | Update a notification subscription



## create_notification_subscription1

> crate::models::NotificationSubscription create_notification_subscription1(project_id, create_notification_subscription)
Create a notification subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project | [required] |
**create_notification_subscription** | Option<[**CreateNotificationSubscription**](CreateNotificationSubscription.md)> | The new subscription |  |

### Return type

[**crate::models::NotificationSubscription**](NotificationSubscription.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification_subscription1

> delete_notification_subscription1(project_id, subscription_id)
Delete a notification subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project | [required] |
**subscription_id** | **String** | The ID of the notification subscription to delete | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_subscription1

> crate::models::NotificationSubscription get_notification_subscription1(project_id, subscription_id)
Retrieve a notification subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project | [required] |
**subscription_id** | **String** | The ID of the notification subscription | [required] |

### Return type

[**crate::models::NotificationSubscription**](NotificationSubscription.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_subscriptions1

> crate::models::NotificationSubscriptionList get_notification_subscriptions1(project_id)
Retrieve notification subscriptions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project | [required] |

### Return type

[**crate::models::NotificationSubscriptionList**](NotificationSubscriptionList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notification_subscription1

> crate::models::NotificationSubscription update_notification_subscription1(project_id, subscription_id, if_match, notification_subscription)
Update a notification subscription

Fields which can be updated:  - enabled  - eventCode  - filterExpression  - notificationChannel 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The ID of the project | [required] |
**subscription_id** | **String** | The ID of the notification subscription to update | [required] |
**if_match** | Option<**String**> | Optional header parameter to enable conflict exposure. If the client provides this header, then it must contains the client's most recent value of the 'ETag' response header, and the server will respond with a 409 code if it detects a conflict. If the client does not provide this header, then the server will not do a conflict check, which means that as a client you can override the resource even when the server has a more recent version. |  |
**notification_subscription** | Option<[**NotificationSubscription**](NotificationSubscription.md)> | The updated subscription |  |

### Return type

[**crate::models::NotificationSubscription**](NotificationSubscription.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

