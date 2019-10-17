# LoriotApplication

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | Option<**f32**> | Application ID | [optional]
**name** | Option<**String**> | Application name | [optional]
**created** | Option<[**String**](string.md)> | Creation date | [optional]
**devices** | Option<**f32**> | Amount of devices registered for the application | [optional]
**device_limit** | Option<**f32**> | Limit of devices which can be registered | [optional]
**output** | Option<**String**> | Data Output name to be used to publish LoRaWAN information | [optional]
**osetup** | Option<[**serde_json::Value**](.md)> | Internal configuration for selected Data Output | [optional]
**overbosity** | Option<**String**> | Amount of verbosity for LoRaWAN information | [optional]
**ogwinfo** | Option<**String**> | Gateway information feed verbosity | [optional]
**suspended** | Option<**bool**> | Suspended application status | [optional]
**orx** | Option<**bool**> | Enable/disable RX messages output | [optional]
**appeui** | Option<**String**> | Application EUI | [optional]
**join_server_id** | Option<**String**> | Join Server EUI to handle JoinReq messages | [optional]
**canotaa** | Option<**bool**> | Flag to enable/disable usage of OTAA JoinReq messages | [optional]
**publish_app_s_key** | Option<**bool**> | Flag to publish AppSKey after successful JoinReq | [optional]
**clients_limit** | Option<**f32**> | Maximum number of concurrent connections to Data Output | [optional]
**custom_properties_uplink_data_message** | Option<**bool**> | Status flag to enable/disable sending custom properties to be published\\ for each uplink data message  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


