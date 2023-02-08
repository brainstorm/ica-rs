# \SubscriptionsApi

All URIs are relative to *https://aps2.platform.illumina.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_subscription**](SubscriptionsApi.md#create_subscription) | **POST** /v1/subscriptions | Creates a subscription to an event type and defines how those events get delivered.
[**disable_subscription**](SubscriptionsApi.md#disable_subscription) | **DELETE** /v1/subscriptions/{subscriptionId} | Given a subscription id, disables the specified subscription.
[**get_subscription**](SubscriptionsApi.md#get_subscription) | **GET** /v1/subscriptions/{subscriptionId} | Given a subscription id, returns information about that subscription.
[**list_subscriptions**](SubscriptionsApi.md#list_subscriptions) | **GET** /v1/subscriptions | Get a list of subscriptions.



## create_subscription

> crate::models::Subscription create_subscription(body)
Creates a subscription to an event type and defines how those events get delivered.

Events can be delivered to AWS SQS, AWS SNS, or can be used to launch a WES workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateSubscriptionRequest**](CreateSubscriptionRequest.md)> |  |  |

### Return type

[**crate::models::Subscription**](Subscription.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_subscription

> crate::models::Subscription disable_subscription(subscription_id)
Given a subscription id, disables the specified subscription.

Given a subscription id, disables that subscription with the current JWT token’s tenant Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | Id of the subscription to be disabled | [required] |

### Return type

[**crate::models::Subscription**](Subscription.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription

> crate::models::Subscription get_subscription(subscription_id)
Given a subscription id, returns information about that subscription.

Given a subscription id, returns information about that subscription accessible by the current JWT token’s tenant Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | Id of the subscription to return | [required] |

### Return type

[**crate::models::Subscription**](Subscription.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subscriptions

> crate::models::SubscriptionList list_subscriptions(event_type, page_size, page_token)
Get a list of subscriptions.

Get a list of subscriptions accessible by the current JWT token’s tenant Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_type** | Option<**String**> | Optional event type for filtering returned subscriptions |  |
**page_size** | Option<**i32**> | Optional parameter to define the page size returned. Valid inputs range from 1-1000. |  |
**page_token** | Option<**String**> | Utilized for navigation after initial listing. Valid values include those of  firstPageToken, nextPageToken, and previousPageToken in the list response.  When there are no more pages, the nextPageToken will be null. |  |

### Return type

[**crate::models::SubscriptionList**](SubscriptionList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

