# \ApplicationOutputApi

All URIs are relative to *https://eu1.loriot.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**1_nwk_app_appid_outputs_get**](ApplicationOutputApi.md#1_nwk_app_appid_outputs_get) | **get** /1/nwk/app/{APPID}/outputs | Retrieve application data output type definition for each data output in use
[**1_nwk_app_appid_outputs_outputid_delete**](ApplicationOutputApi.md#1_nwk_app_appid_outputs_outputid_delete) | **delete** /1/nwk/app/{APPID}/outputs/{OUTPUTID} | Remove selected Data Output by its ID
[**1_nwk_app_appid_outputs_post**](ApplicationOutputApi.md#1_nwk_app_appid_outputs_post) | **post** /1/nwk/app/{APPID}/outputs | Add new Data Output to current application
[**1_nwk_apps_output_get**](ApplicationOutputApi.md#1_nwk_apps_output_get) | **get** /1/nwk/apps/output | Retrieve list of compatible output types
[**1_nwk_apps_output_outputid_get**](ApplicationOutputApi.md#1_nwk_apps_output_outputid_get) | **get** /1/nwk/apps/output/{OUTPUTID} | Retrieve detailed information about application output



## 1_nwk_app_appid_outputs_get

> Vec<crate::models::LoriotAppDataOutputType> 1_nwk_app_appid_outputs_get(APPID)
Retrieve application data output type definition for each data output in use

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |

### Return type

[**Vec<crate::models::LoriotAppDataOutputType>**](AppDataOutputType.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_outputs_outputid_delete

> 1_nwk_app_appid_outputs_outputid_delete(APPID, OUTPUTID)
Remove selected Data Output by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**OUTPUTID** | **String** | Data Output ID to remove | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_app_appid_outputs_post

> Vec<crate::models::LoriotOutputDetail> 1_nwk_app_appid_outputs_post(APPID, loriot_inline_object1)
Add new Data Output to current application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**APPID** | **String** | Application hexadecimal (uppercase) ID | [required] |
**loriot_inline_object1** | [**LoriotInlineObject1**](LoriotInlineObject1.md) |  | [required] |

### Return type

[**Vec<crate::models::LoriotOutputDetail>**](OutputDetail.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_apps_output_get

> Vec<crate::models::LoriotOutputDetail> 1_nwk_apps_output_get()
Retrieve list of compatible output types

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LoriotOutputDetail>**](OutputDetail.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## 1_nwk_apps_output_outputid_get

> crate::models::LoriotOutputDetail 1_nwk_apps_output_outputid_get(OUTPUTID)
Retrieve detailed information about application output

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**OUTPUTID** | **String** | Data Output ID to remove | [required] |

### Return type

[**crate::models::LoriotOutputDetail**](OutputDetail.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

