# \NetworkApi

All URIs are relative to *https://eu1.loriot.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**1_nwk_network_roamingid_delete**](NetworkApi.md#1_nwk_network_roamingid_delete) | **delete** /1/nwk/network/{roamingid} | Remove the Gateway Network
[**1_nwk_network_roamingid_gateway_gweui_move_targetroamingid_put**](NetworkApi.md#1_nwk_network_roamingid_gateway_gweui_move_targetroamingid_put) | **put** /1/nwk/network/{roamingid}/gateway/{gweui}/move/{targetroamingid} | Move gateway from source gateway network to target gateway network. Both gateway networks must belong to current user.
[**1_nwk_network_roamingid_gateways_get**](NetworkApi.md#1_nwk_network_roamingid_gateways_get) | **get** /1/nwk/network/{roamingid}/gateways | Retrieve network's gateways
[**1_nwk_network_roamingid_gateways_post**](NetworkApi.md#1_nwk_network_roamingid_gateways_post) | **post** /1/nwk/network/{roamingid}/gateways | Register new gateway and add it to a network
[**1_nwk_network_roamingid_get**](NetworkApi.md#1_nwk_network_roamingid_get) | **get** /1/nwk/network/{roamingid} | Retrieve Network details (without gateways information)
[**1_nwk_network_roamingid_location_put**](NetworkApi.md#1_nwk_network_roamingid_location_put) | **put** /1/nwk/network/{roamingid}/location | Modify the network location
[**1_nwk_network_roamingid_put**](NetworkApi.md#1_nwk_network_roamingid_put) | **put** /1/nwk/network/{roamingid} | Modify any network detail
[**1_nwk_network_roamingid_visibility_put**](NetworkApi.md#1_nwk_network_roamingid_visibility_put) | **put** /1/nwk/network/{roamingid}/visibility | Modify the network **visibility** and the gateways visbility inside the network
[**1_nwk_networks_get**](NetworkApi.md#1_nwk_networks_get) | **get** /1/nwk/networks | Retrieve all gateway networks for current user
[**1_nwk_networks_post**](NetworkApi.md#1_nwk_networks_post) | **post** /1/nwk/networks | Creates new Gateway Network for current user



## 1_nwk_network_roamingid_delete

> 1_nwk_network_roamingid_delete(roamingid)
Remove the Gateway Network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**roamingid** | **String** | Gateway Network Roaming ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_network_roamingid_gateway_gweui_move_targetroamingid_put

> 1_nwk_network_roamingid_gateway_gweui_move_targetroamingid_put(roamingid, gweui, targetroamingid, body)
Move gateway from source gateway network to target gateway network. Both gateway networks must belong to current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**roamingid** | **String** | Gateway Network Roaming ID | [required] |
**gweui** | **String** | Gateway EUI | [required] |
**targetroamingid** | **String** | Gateway Network Roaming ID Gateway's move target | [required] |
**body** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_network_roamingid_gateways_get

> crate::models::LoriotInlineResponse20012 1_nwk_network_roamingid_gateways_get(roamingid, page, per_page, filter, sort)
Retrieve network's gateways

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**roamingid** | **String** | Gateway Network Roaming ID | [required] |
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |

### Return type

[**crate::models::LoriotInlineResponse20012**](inline_response_200_12.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_network_roamingid_gateways_post

> crate::models::LoriotInlineResponse20013 1_nwk_network_roamingid_gateways_post(roamingid, loriot_inline_object20)
Register new gateway and add it to a network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**roamingid** | **String** | Gateway Network Roaming ID | [required] |
**loriot_inline_object20** | Option<[**LoriotInlineObject20**](LoriotInlineObject20.md)> |  |  |

### Return type

[**crate::models::LoriotInlineResponse20013**](inline_response_200_13.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_network_roamingid_get

> crate::models::LoriotGatewayNetwork 1_nwk_network_roamingid_get(roamingid)
Retrieve Network details (without gateways information)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**roamingid** | **String** | Gateway Network Roaming ID | [required] |

### Return type

[**crate::models::LoriotGatewayNetwork**](GatewayNetwork.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_network_roamingid_location_put

> 1_nwk_network_roamingid_location_put(roamingid, loriot_inline_object21)
Modify the network location

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**roamingid** | **String** | Gateway Network Roaming ID | [required] |
**loriot_inline_object21** | Option<[**LoriotInlineObject21**](LoriotInlineObject21.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_network_roamingid_put

> 1_nwk_network_roamingid_put(roamingid, loriot_gateway_network)
Modify any network detail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**roamingid** | **String** | Gateway Network Roaming ID | [required] |
**loriot_gateway_network** | Option<[**LoriotGatewayNetwork**](LoriotGatewayNetwork.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_network_roamingid_visibility_put

> 1_nwk_network_roamingid_visibility_put(roamingid, loriot_inline_object22)
Modify the network **visibility** and the gateways visbility inside the network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**roamingid** | **String** | Gateway Network Roaming ID | [required] |
**loriot_inline_object22** | Option<[**LoriotInlineObject22**](LoriotInlineObject22.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_networks_get

> crate::models::LoriotInlineResponse20014 1_nwk_networks_get(page, per_page, filter, sort)
Retrieve all gateway networks for current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **f32** | For server-side pagination purpose, this is the page number for the response | [required] |
**per_page** | **f32** | Amount of registers that the API should show in the response | [required] |
**filter** | Option<**String**> | Filter property values |  |
**sort** | Option<**String**> | The parameter that should be used as order by |  |

### Return type

[**crate::models::LoriotInlineResponse20014**](inline_response_200_14.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_networks_post

> crate::models::LoriotGatewayNetwork 1_nwk_networks_post(loriot_inline_object23)
Creates new Gateway Network for current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loriot_inline_object23** | Option<[**LoriotInlineObject23**](LoriotInlineObject23.md)> |  |  |

### Return type

[**crate::models::LoriotGatewayNetwork**](GatewayNetwork.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

