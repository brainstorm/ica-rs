# \NotificationChannelApi

All URIs are relative to */ica/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_notification_channel**](NotificationChannelApi.md#create_notification_channel) | **POST** /api/notificationChannels | Create a notification channel
[**delete_notification_channel**](NotificationChannelApi.md#delete_notification_channel) | **DELETE** /api/notificationChannels/{channelId} | Delete a notification channel
[**get_notification_channel**](NotificationChannelApi.md#get_notification_channel) | **GET** /api/notificationChannels/{channelId} | Retrieve a notification channel
[**get_notification_channels**](NotificationChannelApi.md#get_notification_channels) | **GET** /api/notificationChannels | Retrieve notification channels
[**update_notification_channel**](NotificationChannelApi.md#update_notification_channel) | **PUT** /api/notificationChannels/{channelId} | Update a notification channel



## create_notification_channel

> crate::models::NotificationChannel create_notification_channel(create_notification_channel)
Create a notification channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_notification_channel** | Option<[**CreateNotificationChannel**](CreateNotificationChannel.md)> | The new channel |  |

### Return type

[**crate::models::NotificationChannel**](NotificationChannel.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification_channel

> delete_notification_channel(channel_id)
Delete a notification channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the notification channel to delete | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_channel

> crate::models::NotificationChannel get_notification_channel(channel_id)
Retrieve a notification channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the notification channel to retrieve | [required] |

### Return type

[**crate::models::NotificationChannel**](NotificationChannel.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_channels

> crate::models::NotificationChannelList get_notification_channels()
Retrieve notification channels

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::NotificationChannelList**](NotificationChannelList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notification_channel

> crate::models::NotificationChannel update_notification_channel(channel_id, if_match, notification_channel)
Update a notification channel

This will affect all subscriptions which use this address!Fields which can be updated:  - enabled  - address 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the notification channel to update | [required] |
**if_match** | Option<**String**> | Optional header parameter to enable conflict exposure. If the client provides this header, then it must contains the client's most recent value of the 'ETag' response header, and the server will respond with a 409 code if it detects a conflict. If the client does not provide this header, then the server will not do a conflict check, which means that as a client you can override the resource even when the server has a more recent version. |  |
**notification_channel** | Option<[**NotificationChannel**](NotificationChannel.md)> | The updated channel |  |

### Return type

[**crate::models::NotificationChannel**](NotificationChannel.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [JwtAuth](../README.md#JwtAuth)

### HTTP request headers

- **Content-Type**: application/vnd.illumina.v3+json, application/json
- **Accept**: application/vnd.illumina.v3+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

