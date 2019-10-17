# \SystemApi

All URIs are relative to *https://eu1.loriot.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**1_nwk_status_get**](SystemApi.md#1_nwk_status_get) | **get** /1/nwk/status | This is a dinamic endpoint. It depends if an authorization is provided. If a valid authorization (sessionId or bearer) is sent, all fields described in this documentation will be received. If there is no authorization in the request, the response comes only with mapsKey and serverType.



## 1_nwk_status_get

> crate::models::LoriotInlineResponse20024 1_nwk_status_get()
This is a dinamic endpoint. It depends if an authorization is provided. If a valid authorization (sessionId or bearer) is sent, all fields described in this documentation will be received. If there is no authorization in the request, the response comes only with mapsKey and serverType.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LoriotInlineResponse20024**](inline_response_200_24.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [sessionAuth](../README.md#sessionAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

