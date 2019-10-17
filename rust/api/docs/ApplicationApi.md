# \ApplicationApi

All URIs are relative to *https://eu1.loriot.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**1_nwk_app_appid_app_delete**](ApplicationApi.md#1_nwk_app_appid_app_delete) | **delete** /1/nwk/app/{APPID}/app | Remove application, application statistics, application LoRaWAN data, enqueued downlink messages and devices registered
[**1_nwk_app_appid_app_get**](ApplicationApi.md#1_nwk_app_appid_app_get) | **get** /1/nwk/app/{APPID}/app | Retrieve application properties by its ID
[**1_nwk_app_appid_app_post**](ApplicationApi.md#1_nwk_app_appid_app_post) | **post** /1/nwk/app/{APPID}/app | Modify application properties
[**1_nwk_app_appid_capacity_post**](ApplicationApi.md#1_nwk_app_appid_capacity_post) | **post** /1/nwk/app/{APPID}/capacity | Change the allocated capacity of the application
[**1_nwk_app_appid_last_data_get**](ApplicationApi.md#1_nwk_app_appid_last_data_get) | **get** /1/nwk/app/{APPID}/last_data | Last 25 frames received by the application
[**1_nwk_app_appid_log_get**](ApplicationApi.md#1_nwk_app_appid_log_get) | **get** /1/nwk/app/{APPID}/log | Retrieve application events logs (using pagination by default)
[**1_nwk_app_appid_title_post**](ApplicationApi.md#1_nwk_app_appid_title_post) | **post** /1/nwk/app/{APPID}/title | Change the title of the application
[**1_nwk_app_appid_token_get**](ApplicationApi.md#1_nwk_app_appid_token_get) | **get** /1/nwk/app/{APPID}/token | Retrieves application token
[**1_nwk_app_appid_token_post**](ApplicationApi.md#1_nwk_app_appid_token_post) | **post** /1/nwk/app/{APPID}/token | Generate Authentication Token
[**1_nwk_app_appid_token_token_post**](ApplicationApi.md#1_nwk_app_appid_token_token_post) | **post** /1/nwk/app/{APPID}/token/{TOKEN} | Generate Authentication Token
[**1_nwk_app_appid_traffic_freq_get**](ApplicationApi.md#1_nwk_app_appid_traffic_freq_get) | **get** /1/nwk/app/{APPID}/traffic_{FREQ} | Retrieve uplink and downlink data traffic statistics for the application
[**1_nwk_apps_get**](ApplicationApi.md#1_nwk_apps_get) | **get** /1/nwk/apps | Retrieve user applications using pagination
[**1_nwk_apps_post**](ApplicationApi.md#1_nwk_apps_post) | **post** /1/nwk/apps | Create new application
[**1_nwk_apps_usage_get**](ApplicationApi.md#1_nwk_apps_usage_get) | **get** /1/nwk/apps/usage | Retrieve user applications usage



## 1_nwk_app_appid_app_delete

> 1_nwk_app_appid_app_delete(APPID)
Remove application, application statistics, application LoRaWAN data, enqueued downlink messages and devices registered

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_app_get

> crate::models::LoriotApplication 1_nwk_app_appid_app_get(APPID)
Retrieve application properties by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |

### Return type

[**crate::models::LoriotApplication**](Application.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_app_post

> 1_nwk_app_appid_app_post(APPID, loriot_inline_object3)
Modify application properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**loriot_inline_object3** | Option<[**LoriotInlineObject3**](LoriotInlineObject3.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_capacity_post

> crate::models::LoriotInlineResponse2004 1_nwk_app_appid_capacity_post(APPID, loriot_inline_object5)
Change the allocated capacity of the application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**loriot_inline_object5** | [**LoriotInlineObject5**](LoriotInlineObject5.md) |  | [required] |

### Return type

[**crate::models::LoriotInlineResponse2004**](inline_response_200_4.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_last_data_get

> Vec<crate::models::LoriotLastData> 1_nwk_app_appid_last_data_get(APPID)
Last 25 frames received by the application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |

### Return type

[**Vec<crate::models::LoriotLastData>**](LastData.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_log_get

> crate::models::LoriotInlineResponse2002 1_nwk_app_appid_log_get(APPID, page, per_page, filetype, filter, sort)
Retrieve application events logs (using pagination by default)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
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


## 1_nwk_app_appid_title_post

> crate::models::LoriotInlineResponse2003 1_nwk_app_appid_title_post(APPID, loriot_inline_object4)
Change the title of the application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**loriot_inline_object4** | [**LoriotInlineObject4**](LoriotInlineObject4.md) |  | [required] |

### Return type

[**crate::models::LoriotInlineResponse2003**](inline_response_200_3.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_token_get

> Vec<String> 1_nwk_app_appid_token_get(APPID)
Retrieves application token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |

### Return type

**Vec<String>**

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_token_post

> crate::models::LoriotInlineResponse2006 1_nwk_app_appid_token_post(APPID, body)
Generate Authentication Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::LoriotInlineResponse2006**](inline_response_200_6.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_token_token_post

> 1_nwk_app_appid_token_token_post(APPID, TOKEN, body)
Generate Authentication Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**TOKEN** | **String** | Application Token | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_traffic_freq_get

> crate::models::LoriotInlineResponse2005 1_nwk_app_appid_traffic_freq_get(APPID, FREQ)
Retrieve uplink and downlink data traffic statistics for the application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**FREQ** | **String** | Amount and resolution of data returned | [required] |

### Return type

[**crate::models::LoriotInlineResponse2005**](inline_response_200_5.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_apps_get

> crate::models::LoriotPaginationApps 1_nwk_apps_get(page, per_page, filter, sort)
Retrieve user applications using pagination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |

### Return type

[**crate::models::LoriotPaginationApps**](PaginationApps.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_apps_post

> crate::models::LoriotApplication 1_nwk_apps_post(loriot_inline_object)
Create new application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loriot_inline_object** | Option<[**LoriotInlineObject**](LoriotInlineObject.md)> |  |  |

### Return type

[**crate::models::LoriotApplication**](Application.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_apps_usage_get

> crate::models::LoriotInlineResponse200 1_nwk_apps_usage_get()
Retrieve user applications usage

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LoriotInlineResponse200**](inline_response_200.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

