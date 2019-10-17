# LoriotInlineObject10

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rxw** | Option<**f32**> | Receive window to be used after uplink transmission | [optional]
**seqrelax** | Option<**bool**> | Flag to relax or strict uplink seqno check | [optional]
**devclass** | Option<**String**> | Device LoRaWAN class type | [optional]
**adr_min** | Option<**f32**> | Minimum data rate to use for device when ADR is enabled | [optional]
**adr_max** | Option<**f32**> | Maximum data rate to use for device when ADR is enabled | [optional]
**adr_fix** | Option<**f32**> | Fixed value for data rate when ADR is enabled | [optional]
**title** | Option<**String**> | Device title, EUI by default | [optional]
**description** | Option<**String**> | Device description, empty by default | [optional]
**seqdnreset** | Option<**bool**> | Flag to reset sequence downlink when old uplink seqno received | [optional]
**can_send** | Option<**bool**> | Enable/disable downlinks for device | [optional]
**can_send_payload** | Option<**bool**> | Enable/disable payload in a downlink for device | [optional]
**can_send_fopts** | Option<**bool**> | Enable/disable MAC in a downlink for device | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


