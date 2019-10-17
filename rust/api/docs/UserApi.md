# \UserApi

All URIs are relative to *https://eu1.loriot.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**1_nwk_user_alerts_post**](UserApi.md#1_nwk_user_alerts_post) | **post** /1/nwk/user/alerts | Mechanism for notifying users about their gateway’s status changes
[**1_nwk_user_email_notification_delete**](UserApi.md#1_nwk_user_email_notification_delete) | **delete** /1/nwk/user/email-notification | Delete the user email notification. Default user email will be used for notifications.
[**1_nwk_user_email_notification_get**](UserApi.md#1_nwk_user_email_notification_get) | **get** /1/nwk/user/email-notification | Retrieve current user email notification
[**1_nwk_user_email_notification_put**](UserApi.md#1_nwk_user_email_notification_put) | **put** /1/nwk/user/email-notification | Modify the user email notification
[**1_nwk_user_email_put**](UserApi.md#1_nwk_user_email_put) | **put** /1/nwk/user/email | Modify the user **email** if the email address is not used in a different account
[**1_nwk_user_get**](UserApi.md#1_nwk_user_get) | **get** /1/nwk/user | Retrieve current user information
[**1_nwk_user_usage_get**](UserApi.md#1_nwk_user_usage_get) | **get** /1/nwk/user/usage | Detailed usage of user resources and his limits
[**1_nwk_user_webhook_notification_delete**](UserApi.md#1_nwk_user_webhook_notification_delete) | **delete** /1/nwk/user/webhook-notification | Delete the user webhook notification. Default user webhook will be used for notifications.
[**1_nwk_user_webhook_notification_get**](UserApi.md#1_nwk_user_webhook_notification_get) | **get** /1/nwk/user/webhook-notification | Retrieve current user webhook notification
[**1_nwk_user_webhook_notification_put**](UserApi.md#1_nwk_user_webhook_notification_put) | **put** /1/nwk/user/webhook-notification | Modify the user webhook notification



## 1_nwk_user_alerts_post

> crate::models::LoriotInlineResponse204 1_nwk_user_alerts_post(body)
Mechanism for notifying users about their gateway’s status changes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**bool**> |  |  |

### Return type

[**crate::models::LoriotInlineResponse204**](inline_response_204.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_user_email_notification_delete

> 1_nwk_user_email_notification_delete()
Delete the user email notification. Default user email will be used for notifications.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_user_email_notification_get

> crate::models::LoriotUserEmailNotification 1_nwk_user_email_notification_get()
Retrieve current user email notification

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LoriotUserEmailNotification**](UserEmailNotification.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_user_email_notification_put

> crate::models::LoriotUserEmailNotification 1_nwk_user_email_notification_put(loriot_inline_object17)
Modify the user email notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loriot_inline_object17** | Option<[**LoriotInlineObject17**](LoriotInlineObject17.md)> |  |  |

### Return type

[**crate::models::LoriotUserEmailNotification**](UserEmailNotification.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_user_email_put

> 1_nwk_user_email_put(loriot_inline_object19)
Modify the user **email** if the email address is not used in a different account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loriot_inline_object19** | Option<[**LoriotInlineObject19**](LoriotInlineObject19.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_user_get

> crate::models::LoriotUser 1_nwk_user_get()
Retrieve current user information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LoriotUser**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_user_usage_get

> crate::models::LoriotInlineResponse20011 1_nwk_user_usage_get()
Detailed usage of user resources and his limits

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LoriotInlineResponse20011**](inline_response_200_11.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_user_webhook_notification_delete

> 1_nwk_user_webhook_notification_delete()
Delete the user webhook notification. Default user webhook will be used for notifications.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_user_webhook_notification_get

> crate::models::LoriotUserWebhookNotification 1_nwk_user_webhook_notification_get()
Retrieve current user webhook notification

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LoriotUserWebhookNotification**](UserWebhookNotification.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_user_webhook_notification_put

> crate::models::LoriotUserWebhookNotification 1_nwk_user_webhook_notification_put(loriot_inline_object18)
Modify the user webhook notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loriot_inline_object18** | Option<[**LoriotInlineObject18**](LoriotInlineObject18.md)> |  |  |

### Return type

[**crate::models::LoriotUserWebhookNotification**](UserWebhookNotification.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

