# \DevicesApi

All URIs are relative to *https://eu1.loriot.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**1_nwk_app_appid_device_deveui_appkey_delete**](DevicesApi.md#1_nwk_app_appid_device_deveui_appkey_delete) | **delete** /1/nwk/app/{APPID}/device/{DEVEUI}/appkey | Delete device's AppKey
[**1_nwk_app_appid_device_deveui_appkey_post**](DevicesApi.md#1_nwk_app_appid_device_deveui_appkey_post) | **post** /1/nwk/app/{APPID}/device/{DEVEUI}/appkey | Set LoRaWAN AppKey for the device. If request body is empty, a key will be generated. 
[**1_nwk_app_appid_device_deveui_appskey_delete**](DevicesApi.md#1_nwk_app_appid_device_deveui_appskey_delete) | **delete** /1/nwk/app/{APPID}/device/{DEVEUI}/appskey | Delete device's AppSKey
[**1_nwk_app_appid_device_deveui_appskey_post**](DevicesApi.md#1_nwk_app_appid_device_deveui_appskey_post) | **post** /1/nwk/app/{APPID}/device/{DEVEUI}/appskey | Set LoRaWAN AppSKey for the device. If request body is empty, a key will be generated. 
[**1_nwk_app_appid_device_deveui_delete**](DevicesApi.md#1_nwk_app_appid_device_deveui_delete) | **delete** /1/nwk/app/{APPID}/device/{DEVEUI} | Remove device
[**1_nwk_app_appid_device_deveui_dutycycle_post**](DevicesApi.md#1_nwk_app_appid_device_deveui_dutycycle_post) | **post** /1/nwk/app/{APPID}/device/{DEVEUI}/dutycycle | Set LoRaWAN duty cycle
[**1_nwk_app_appid_device_deveui_flushqueue_post**](DevicesApi.md#1_nwk_app_appid_device_deveui_flushqueue_post) | **post** /1/nwk/app/{APPID}/device/{DEVEUI}/flushqueue | Clear all queued downlink messages for the device
[**1_nwk_app_appid_device_deveui_get**](DevicesApi.md#1_nwk_app_appid_device_deveui_get) | **get** /1/nwk/app/{APPID}/device/{DEVEUI} | Retrieve device properties
[**1_nwk_app_appid_device_deveui_last_data_get**](DevicesApi.md#1_nwk_app_appid_device_deveui_last_data_get) | **get** /1/nwk/app/{APPID}/device/{DEVEUI}/last_data | Retrieve last 10 data frames received from the device
[**1_nwk_app_appid_device_deveui_log_get**](DevicesApi.md#1_nwk_app_appid_device_deveui_log_get) | **get** /1/nwk/app/{APPID}/device/{DEVEUI}/log | Retrieve paged event log for the device
[**1_nwk_app_appid_device_deveui_mac_commands_get**](DevicesApi.md#1_nwk_app_appid_device_deveui_mac_commands_get) | **get** /1/nwk/app/{APPID}/device/{DEVEUI}/mac-commands | Retrieve paged history of MAC commands sent/received by device
[**1_nwk_app_appid_device_deveui_post**](DevicesApi.md#1_nwk_app_appid_device_deveui_post) | **post** /1/nwk/app/{APPID}/device/{DEVEUI} | Update device properties
[**1_nwk_app_appid_device_deveui_radio_freq_get**](DevicesApi.md#1_nwk_app_appid_device_deveui_radio_freq_get) | **get** /1/nwk/app/{APPID}/device/{DEVEUI}/radio_{FREQ} | Retrieve device's radio history statistics
[**1_nwk_app_appid_device_deveui_seqdn_post**](DevicesApi.md#1_nwk_app_appid_device_deveui_seqdn_post) | **post** /1/nwk/app/{APPID}/device/{DEVEUI}/seqdn | Resets downlink sequence number (LoRaWAN FCntDown).
[**1_nwk_app_appid_device_deveui_seqno_post**](DevicesApi.md#1_nwk_app_appid_device_deveui_seqno_post) | **post** /1/nwk/app/{APPID}/device/{DEVEUI}/seqno | Reset uplink sequence number (LoRaWAN FCntUp)
[**1_nwk_app_appid_device_deveui_tag_tagname_delete**](DevicesApi.md#1_nwk_app_appid_device_deveui_tag_tagname_delete) | **delete** /1/nwk/app/{APPID}/device/{DEVEUI}/tag/{TAGNAME} | Remove tag from device
[**1_nwk_app_appid_device_deveui_tag_tagname_post**](DevicesApi.md#1_nwk_app_appid_device_deveui_tag_tagname_post) | **post** /1/nwk/app/{APPID}/device/{DEVEUI}/tag/{TAGNAME} | Add new tag to device
[**1_nwk_app_appid_device_deveui_tags_get**](DevicesApi.md#1_nwk_app_appid_device_deveui_tags_get) | **get** /1/nwk/app/{APPID}/device/{DEVEUI}/tags/ | Retrieve tags assigned to device
[**1_nwk_app_appid_device_deveui_traffic_freq_get**](DevicesApi.md#1_nwk_app_appid_device_deveui_traffic_freq_get) | **get** /1/nwk/app/{APPID}/device/{DEVEUI}/traffic_{FREQ} | Retrieve device's traffic history statistics
[**1_nwk_app_appid_devices_abp_post**](DevicesApi.md#1_nwk_app_appid_devices_abp_post) | **post** /1/nwk/app/{APPID}/devices/abp | Register new LoRaWAN ABP device.
[**1_nwk_app_appid_devices_get**](DevicesApi.md#1_nwk_app_appid_devices_get) | **get** /1/nwk/app/{APPID}/devices | Retrieve paged devices of the application
[**1_nwk_app_appid_devices_otaa_post**](DevicesApi.md#1_nwk_app_appid_devices_otaa_post) | **post** /1/nwk/app/{APPID}/devices/otaa | Register new LoRaWAN OTAA device.
[**1_nwk_app_appid_devices_post**](DevicesApi.md#1_nwk_app_appid_devices_post) | **post** /1/nwk/app/{APPID}/devices | Generate new LoRaWAN device parameters



## 1_nwk_app_appid_device_deveui_appkey_delete

> 1_nwk_app_appid_device_deveui_appkey_delete(APPID, DEVEUI)
Delete device's AppKey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_appkey_post

> crate::models::LoriotInlineResponse2009 1_nwk_app_appid_device_deveui_appkey_post(APPID, DEVEUI, loriot_inline_object11)
Set LoRaWAN AppKey for the device. If request body is empty, a key will be generated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**loriot_inline_object11** | Option<[**LoriotInlineObject11**](LoriotInlineObject11.md)> |  |  |

### Return type

[**crate::models::LoriotInlineResponse2009**](inline_response_200_9.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_appskey_delete

> 1_nwk_app_appid_device_deveui_appskey_delete(APPID, DEVEUI)
Delete device's AppSKey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_appskey_post

> crate::models::LoriotInlineResponse20010 1_nwk_app_appid_device_deveui_appskey_post(APPID, DEVEUI, loriot_inline_object12)
Set LoRaWAN AppSKey for the device. If request body is empty, a key will be generated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**loriot_inline_object12** | Option<[**LoriotInlineObject12**](LoriotInlineObject12.md)> |  |  |

### Return type

[**crate::models::LoriotInlineResponse20010**](inline_response_200_10.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_delete

> 1_nwk_app_appid_device_deveui_delete(APPID, DEVEUI)
Remove device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_dutycycle_post

> crate::models::LoriotInlineResponse20010 1_nwk_app_appid_device_deveui_dutycycle_post(APPID, DEVEUI, loriot_inline_object13)
Set LoRaWAN duty cycle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**loriot_inline_object13** | Option<[**LoriotInlineObject13**](LoriotInlineObject13.md)> |  |  |

### Return type

[**crate::models::LoriotInlineResponse20010**](inline_response_200_10.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_flushqueue_post

> 1_nwk_app_appid_device_deveui_flushqueue_post(APPID, DEVEUI, loriot_inline_object16)
Clear all queued downlink messages for the device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**loriot_inline_object16** | Option<[**LoriotInlineObject16**](LoriotInlineObject16.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_get

> crate::models::LoriotDevice 1_nwk_app_appid_device_deveui_get(APPID, DEVEUI)
Retrieve device properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |

### Return type

[**crate::models::LoriotDevice**](Device.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_last_data_get

> Vec<crate::models::LoriotLastDataDevice> 1_nwk_app_appid_device_deveui_last_data_get(APPID, DEVEUI)
Retrieve last 10 data frames received from the device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |

### Return type

[**Vec<crate::models::LoriotLastDataDevice>**](LastDataDevice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_log_get

> crate::models::LoriotInlineResponse2002 1_nwk_app_appid_device_deveui_log_get(APPID, DEVEUI, page, per_page, filetype, filter, sort)
Retrieve paged event log for the device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**filetype** | Option<**String**> | Retrieve all logs in JSON format (no pagination) |  |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |

### Return type

[**crate::models::LoriotInlineResponse2002**](inline_response_200_2.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_mac_commands_get

> crate::models::LoriotDeviceMacCommandPaged 1_nwk_app_appid_device_deveui_mac_commands_get(page, per_page, APPID, DEVEUI, filter, sort, command)
Retrieve paged history of MAC commands sent/received by device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |
**command** | Option<**String**> | MAC command name to filter |  |

### Return type

[**crate::models::LoriotDeviceMacCommandPaged**](deviceMacCommandPaged.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_post

> 1_nwk_app_appid_device_deveui_post(APPID, DEVEUI, loriot_inline_object10)
Update device properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**loriot_inline_object10** | Option<[**LoriotInlineObject10**](LoriotInlineObject10.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_radio_freq_get

> Vec<crate::models::LoriotInlineResponse2007> 1_nwk_app_appid_device_deveui_radio_freq_get(APPID, DEVEUI, FREQ)
Retrieve device's radio history statistics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**FREQ** | **String** | Amount and resolution of data returned | [required] |

### Return type

[**Vec<crate::models::LoriotInlineResponse2007>**](inline_response_200_7.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_seqdn_post

> 1_nwk_app_appid_device_deveui_seqdn_post(APPID, DEVEUI, loriot_inline_object15)
Resets downlink sequence number (LoRaWAN FCntDown).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**loriot_inline_object15** | Option<[**LoriotInlineObject15**](LoriotInlineObject15.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_seqno_post

> 1_nwk_app_appid_device_deveui_seqno_post(APPID, DEVEUI, loriot_inline_object14)
Reset uplink sequence number (LoRaWAN FCntUp)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**loriot_inline_object14** | Option<[**LoriotInlineObject14**](LoriotInlineObject14.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_tag_tagname_delete

> 1_nwk_app_appid_device_deveui_tag_tagname_delete(APPID, DEVEUI, TAGNAME)
Remove tag from device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**TAGNAME** | **String** | Tag name | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_tag_tagname_post

> 1_nwk_app_appid_device_deveui_tag_tagname_post(APPID, DEVEUI, TAGNAME, loriot_inline_object9)
Add new tag to device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**TAGNAME** | **String** | Tag name | [required] |
**loriot_inline_object9** | Option<[**LoriotInlineObject9**](LoriotInlineObject9.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_tags_get

> Vec<crate::models::LoriotDevicetag> 1_nwk_app_appid_device_deveui_tags_get(APPID, DEVEUI)
Retrieve tags assigned to device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |

### Return type

[**Vec<crate::models::LoriotDevicetag>**](devicetag.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_device_deveui_traffic_freq_get

> Vec<crate::models::LoriotInlineResponse2008> 1_nwk_app_appid_device_deveui_traffic_freq_get(APPID, DEVEUI, FREQ)
Retrieve device's traffic history statistics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**DEVEUI** | **String** | Device EUI | [required] |
**FREQ** | **String** | Amount and resolution of data returned | [required] |

### Return type

[**Vec<crate::models::LoriotInlineResponse2008>**](inline_response_200_8.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_devices_abp_post

> crate::models::LoriotDevice 1_nwk_app_appid_devices_abp_post(APPID, loriot_inline_object7)
Register new LoRaWAN ABP device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**loriot_inline_object7** | Option<[**LoriotInlineObject7**](LoriotInlineObject7.md)> |  |  |

### Return type

[**crate::models::LoriotDevice**](Device.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_devices_get

> crate::models::LoriotPaginationDevices 1_nwk_app_appid_devices_get(APPID, page, per_page, filter, sort)
Retrieve paged devices of the application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |

### Return type

[**crate::models::LoriotPaginationDevices**](PaginationDevices.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_devices_otaa_post

> crate::models::LoriotDevice 1_nwk_app_appid_devices_otaa_post(APPID, loriot_inline_object8)
Register new LoRaWAN OTAA device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**loriot_inline_object8** | Option<[**LoriotInlineObject8**](LoriotInlineObject8.md)> |  |  |

### Return type

[**crate::models::LoriotDevice**](Device.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_devices_post

> crate::models::LoriotDevice 1_nwk_app_appid_devices_post(APPID, loriot_inline_object6)
Generate new LoRaWAN device parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**loriot_inline_object6** | Option<[**LoriotInlineObject6**](LoriotInlineObject6.md)> |  |  |

### Return type

[**crate::models::LoriotDevice**](Device.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

