# \GatewayApi

All URIs are relative to *https://eu1.loriot.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**1_nwk_gateway_gweui_alerts_get**](GatewayApi.md#1_nwk_gateway_gweui_alerts_get) | **get** /1/nwk/gateway/{gweui}/alerts | Get history of generated gateway alerts
[**1_nwk_gateway_gweui_delete**](GatewayApi.md#1_nwk_gateway_gweui_delete) | **delete** /1/nwk/gateway/{gweui} | Remove user gateway
[**1_nwk_gateway_gweui_get**](GatewayApi.md#1_nwk_gateway_gweui_get) | **get** /1/nwk/gateway/{gweui} | Retrieve user gateway
[**1_nwk_gateway_gweui_gps_get**](GatewayApi.md#1_nwk_gateway_gweui_gps_get) | **get** /1/nwk/gateway/{gweui}/gps | Retrieve paged GPS values
[**1_nwk_gateway_gweui_log_get**](GatewayApi.md#1_nwk_gateway_gweui_log_get) | **get** /1/nwk/gateway/{gweui}/log | Retrieve gateway events log paged
[**1_nwk_gateway_gweui_ping_post**](GatewayApi.md#1_nwk_gateway_gweui_ping_post) | **post** /1/nwk/gateway/{gweui}/ping | Update gateway properties
[**1_nwk_gateway_gweui_post**](GatewayApi.md#1_nwk_gateway_gweui_post) | **post** /1/nwk/gateway/{gweui} | Update gateway properties
[**1_nwk_gateway_gweui_radioband_post**](GatewayApi.md#1_nwk_gateway_gweui_radioband_post) | **post** /1/nwk/gateway/{gweui}/radioband | Change channel plan radioband
[**1_nwk_gateway_gweui_restart_post**](GatewayApi.md#1_nwk_gateway_gweui_restart_post) | **post** /1/nwk/gateway/{gweui}/restart | Restart gateway
[**1_nwk_gateway_gweui_software_allow_next_update_delete**](GatewayApi.md#1_nwk_gateway_gweui_software_allow_next_update_delete) | **delete** /1/nwk/gateway/{gweui}/software/allow-next-update | Disable to update gateway binary when a new version is available in the Network Server for its model. Only available when gateway automatic updates are disabled by default.
[**1_nwk_gateway_gweui_software_allow_next_update_put**](GatewayApi.md#1_nwk_gateway_gweui_software_allow_next_update_put) | **put** /1/nwk/gateway/{gweui}/software/allow-next-update | Enable to update gateway binary when a new version is available in the Network Server for its model. Only available when gateway automatic updates are disabled by default. After apply the update, any new available update will not be applied as long as the gateway automatic updates are disabled
[**1_nwk_gateway_gweui_software_disable_auto_update_put**](GatewayApi.md#1_nwk_gateway_gweui_software_disable_auto_update_put) | **put** /1/nwk/gateway/{gweui}/software/disable-auto-update | Disable gateway binary automatic updates from Network server for each new version.
[**1_nwk_gateway_gweui_software_enable_auto_update_put**](GatewayApi.md#1_nwk_gateway_gweui_software_enable_auto_update_put) | **put** /1/nwk/gateway/{gweui}/software/enable-auto-update | Enable gateway binary automatic updates from Network server for each new version. That should be default action for any gateway.
[**1_nwk_gateway_gweui_sysinfo_get**](GatewayApi.md#1_nwk_gateway_gweui_sysinfo_get) | **get** /1/nwk/gateway/{gweui}/sysinfo | Get system history
[**1_nwk_gateway_gweui_traffic_freq_get**](GatewayApi.md#1_nwk_gateway_gweui_traffic_freq_get) | **get** /1/nwk/gateway/{gweui}/traffic_{FREQ} | Retrieve gateway traffic
[**1_nwk_gateway_gweui_uptime_get**](GatewayApi.md#1_nwk_gateway_gweui_uptime_get) | **get** /1/nwk/gateway/{gweui}/uptime | Retrieve gateway uptime data
[**1_nwk_gateway_models_get**](GatewayApi.md#1_nwk_gateway_models_get) | **get** /1/nwk/gateway/models | Retrieve gateway models
[**1_nwk_gateways_count_get**](GatewayApi.md#1_nwk_gateways_count_get) | **get** /1/nwk/gateways/count | Count gateway networks for current user
[**1_nwk_gateways_get**](GatewayApi.md#1_nwk_gateways_get) | **get** /1/nwk/gateways | Retrieve user gateways using pagination



## 1_nwk_gateway_gweui_alerts_get

> crate::models::LoriotInlineResponse20022 1_nwk_gateway_gweui_alerts_get(gweui, page, per_page, filetype, filter, sort)
Get history of generated gateway alerts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**filetype** | Option<**String**> | Download alerts history in 'csv' or 'json' format. Pagination will be disabled. |  |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |

### Return type

[**crate::models::LoriotInlineResponse20022**](inline_response_200_22.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_delete

> 1_nwk_gateway_gweui_delete(gweui)
Remove user gateway

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_get

> crate::models::LoriotGateway 1_nwk_gateway_gweui_get(gweui)
Retrieve user gateway

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |

### Return type

[**crate::models::LoriotGateway**](Gateway.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_gps_get

> crate::models::LoriotInlineResponse20021 1_nwk_gateway_gweui_gps_get(gweui, page, per_page, filter, sort)
Retrieve paged GPS values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |

### Return type

[**crate::models::LoriotInlineResponse20021**](inline_response_200_21.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_log_get

> crate::models::LoriotInlineResponse20020 1_nwk_gateway_gweui_log_get(page, per_page, gweui, filter)
Retrieve gateway events log paged

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**gweui** | **String** | Gateway EUI | [required] |
**filter** | Option<**String**> | Filter property values |  |

### Return type

[**crate::models::LoriotInlineResponse20020**](inline_response_200_20.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_ping_post

> 1_nwk_gateway_gweui_ping_post(gweui, body)
Update gateway properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |
**body** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_post

> 1_nwk_gateway_gweui_post(gweui, loriot_inline_object24)
Update gateway properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |
**loriot_inline_object24** | Option<[**LoriotInlineObject24**](LoriotInlineObject24.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_radioband_post

> 1_nwk_gateway_gweui_radioband_post(gweui, loriot_inline_object25)
Change channel plan radioband

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |
**loriot_inline_object25** | Option<[**LoriotInlineObject25**](LoriotInlineObject25.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_restart_post

> 1_nwk_gateway_gweui_restart_post(gweui, body)
Restart gateway

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |
**body** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_software_allow_next_update_delete

> 1_nwk_gateway_gweui_software_allow_next_update_delete(gweui)
Disable to update gateway binary when a new version is available in the Network Server for its model. Only available when gateway automatic updates are disabled by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_software_allow_next_update_put

> 1_nwk_gateway_gweui_software_allow_next_update_put(gweui, body)
Enable to update gateway binary when a new version is available in the Network Server for its model. Only available when gateway automatic updates are disabled by default. After apply the update, any new available update will not be applied as long as the gateway automatic updates are disabled

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |
**body** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_software_disable_auto_update_put

> 1_nwk_gateway_gweui_software_disable_auto_update_put(gweui, body)
Disable gateway binary automatic updates from Network server for each new version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |
**body** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_software_enable_auto_update_put

> 1_nwk_gateway_gweui_software_enable_auto_update_put(gweui, body)
Enable gateway binary automatic updates from Network server for each new version. That should be default action for any gateway.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |
**body** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_sysinfo_get

> Vec<crate::models::LoriotInlineResponse20023> 1_nwk_gateway_gweui_sysinfo_get(gweui, filter, sort)
Get system history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |

### Return type

[**Vec<crate::models::LoriotInlineResponse20023>**](inline_response_200_23.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_traffic_freq_get

> crate::models::LoriotInlineResponse20017 1_nwk_gateway_gweui_traffic_freq_get(gweui, FREQ)
Retrieve gateway traffic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |
**FREQ** | **String** | Amount and resolution of data returned | [required] |

### Return type

[**crate::models::LoriotInlineResponse20017**](inline_response_200_17.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_gweui_uptime_get

> crate::models::LoriotInlineResponse20018 1_nwk_gateway_gweui_uptime_get(gweui)
Retrieve gateway uptime data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gweui** | **String** | Gateway EUI | [required] |

### Return type

[**crate::models::LoriotInlineResponse20018**](inline_response_200_18.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateway_models_get

> crate::models::LoriotInlineResponse20019 1_nwk_gateway_models_get(page, per_page, filter, sort)
Retrieve gateway models

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |

### Return type

[**crate::models::LoriotInlineResponse20019**](inline_response_200_19.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateways_count_get

> crate::models::LoriotInlineResponse20016 1_nwk_gateways_count_get()
Count gateway networks for current user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LoriotInlineResponse20016**](inline_response_200_16.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_gateways_get

> crate::models::LoriotInlineResponse20015 1_nwk_gateways_get(page, per_page, filter, sort)
Retrieve user gateways using pagination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |

### Return type

[**crate::models::LoriotInlineResponse20015**](inline_response_200_15.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

