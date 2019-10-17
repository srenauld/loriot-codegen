# \GatewayChannelPlansApi

All URIs are relative to *https://eu1.loriot.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**1_nwk_channelplans_loragwversion_regions_get**](GatewayChannelPlansApi.md#1_nwk_channelplans_loragwversion_regions_get) | **get** /1/nwk/channelplans/{LORAGWVERSION}/regions | Retrieve regions channels
[**1_nwk_channelplans_loragwversionbandname_channels_get**](GatewayChannelPlansApi.md#1_nwk_channelplans_loragwversionbandname_channels_get) | **get** /1/nwk/channelplans/{LORAGWVERSION}/{BANDNAME}/channels | Retrieve channel plan for a bandname
[**1_nwk_channelplans_loragwversionbandname_rx2_get**](GatewayChannelPlansApi.md#1_nwk_channelplans_loragwversionbandname_rx2_get) | **get** /1/nwk/channelplans/{LORAGWVERSION}/{BANDNAME}/rx2 | Retrieve receive window RX2 for a band name



## 1_nwk_channelplans_loragwversion_regions_get

> Vec<crate::models::LoriotInlineResponse20025> 1_nwk_channelplans_loragwversion_regions_get(LORAGWVERSION)
Retrieve regions channels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**LORAGWVERSION** | **f32** | LoRaWAN gateway version (1, 2) | [required] |

### Return type

[**Vec<crate::models::LoriotInlineResponse20025>**](inline_response_200_25.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_channelplans_loragwversionbandname_channels_get

> Vec<crate::models::LoriotInlineResponse20026> 1_nwk_channelplans_loragwversionbandname_channels_get(LORAGWVERSION, BANDNAME)
Retrieve channel plan for a bandname

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**LORAGWVERSION** | **f32** | LoRaWAN gateway version (1, 2) | [required] |
**BANDNAME** | **String** | Band name | [required] |

### Return type

[**Vec<crate::models::LoriotInlineResponse20026>**](inline_response_200_26.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_channelplans_loragwversionbandname_rx2_get

> crate::models::LoriotInlineResponse20027 1_nwk_channelplans_loragwversionbandname_rx2_get(LORAGWVERSION, BANDNAME)
Retrieve receive window RX2 for a band name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**LORAGWVERSION** | **f32** | LoRaWAN gateway version (1, 2) | [required] |
**BANDNAME** | **String** | Band name | [required] |

### Return type

[**crate::models::LoriotInlineResponse20027**](inline_response_200_27.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

