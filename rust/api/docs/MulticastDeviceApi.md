# \MulticastDeviceApi

All URIs are relative to *https://eu1.loriot.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**1_nwk_app_appid_mcast_device_get**](MulticastDeviceApi.md#1_nwk_app_appid_mcast_device_get) | **get** /1/nwk/app/{APPID}/mcast-device | Only for users from tier 2. Returns all multicast devices for current application.
[**1_nwk_app_appid_mcast_device_mcast_eui_delete**](MulticastDeviceApi.md#1_nwk_app_appid_mcast_device_mcast_eui_delete) | **delete** /1/nwk/app/{APPID}/mcast-device/{mcastEUI} | Only for users from tier 2. Deletes a multicast device
[**1_nwk_app_appid_mcast_device_mcast_eui_put**](MulticastDeviceApi.md#1_nwk_app_appid_mcast_device_mcast_eui_put) | **put** /1/nwk/app/{APPID}/mcast-device/{mcastEUI} | Only for users from tier 2. Update multicast device properties.
[**1_nwk_app_appid_mcast_device_post**](MulticastDeviceApi.md#1_nwk_app_appid_mcast_device_post) | **post** /1/nwk/app/{APPID}/mcast-device | Create new multicast device. Only for users from tier 2. Any future downlink message will be sent to all user's gateways.
[**1_nwk_app_appid_mcastdevlimit_post**](MulticastDeviceApi.md#1_nwk_app_appid_mcastdevlimit_post) | **post** /1/nwk/app/{APPID}/mcastdevlimit | Update multicast device.



## 1_nwk_app_appid_mcast_device_get

> Vec<crate::models::LoriotMcastdev> 1_nwk_app_appid_mcast_device_get(APPID, page, per_page, filter, sort)
Only for users from tier 2. Returns all multicast devices for current application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |

### Return type

[**Vec<crate::models::LoriotMcastdev>**](mcastdev.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_mcast_device_mcast_eui_delete

> 1_nwk_app_appid_mcast_device_mcast_eui_delete(APPID, mcast_eui)
Only for users from tier 2. Deletes a multicast device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**mcast_eui** | **String** | EUI of the multicast device | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_mcast_device_mcast_eui_put

> 1_nwk_app_appid_mcast_device_mcast_eui_put(APPID, mcast_eui, loriot_mcast)
Only for users from tier 2. Update multicast device properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**mcast_eui** | **String** | EUI of the multicast device | [required] |
**loriot_mcast** | Option<[**LoriotMcast**](LoriotMcast.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_mcast_device_post

> crate::models::LoriotMcastdev 1_nwk_app_appid_mcast_device_post(APPID, loriot_mcast)
Create new multicast device. Only for users from tier 2. Any future downlink message will be sent to all user's gateways.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**loriot_mcast** | Option<[**LoriotMcast**](LoriotMcast.md)> |  |  |

### Return type

[**crate::models::LoriotMcastdev**](mcastdev.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_mcastdevlimit_post

> crate::models::LoriotInlineResponse2001 1_nwk_app_appid_mcastdevlimit_post(APPID, loriot_inline_object2)
Update multicast device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**loriot_inline_object2** | Option<[**LoriotInlineObject2**](LoriotInlineObject2.md)> |  |  |

### Return type

[**crate::models::LoriotInlineResponse2001**](inline_response_200_1.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

