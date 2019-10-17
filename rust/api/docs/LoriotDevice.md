# LoriotDevice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | Option<**String**> | Device EUI | [optional]
**devaddr** | Option<**String**> | Device address | [optional]
**seqno** | Option<**f32**> | uplink sequence number | [optional]
**seqdn** | Option<**f32**> | downlink sequence number | [optional]
**seqq** | Option<**f32**> | queued sequence number | [optional]
**adr_cnt** | Option<**f32**> | number of frames since last ADR | [optional]
**subscription** | Option<**f32**> | Subscription type (suspended status) | [optional]
**txrate** | Option<**f32**> |  | [optional]
**rxrate** | Option<**f32**> |  | [optional]
**devclass** | Option<**String**> | Device LoRaWAN class type | [optional]
**rxw** | Option<**f32**> | Receive window to be used after uplink transmission | [optional]
**dutycycle** | Option<**f32**> | Device duty cycle | [optional]
**adr** | Option<**bool**> | Enable/disable adaptative data rate (ADR) for device | [optional]
**adr_min** | Option<**f32**> | Minimum data rate to use for device when ADR is enabled | [optional]
**adr_max** | Option<**f32**> | Maximum data rate to use for device when ADR is enabled | [optional]
**adr_fix** | Option<**f32**> | Fixed value for data rate when ADR is enabled | [optional]
**seqrelax** | Option<**bool**> | Flag to relax or strict uplink seqno check | [optional]
**seqdnreset** | Option<**bool**> | Flag to reset sequence downlink when old uplink seqno received | [optional]
**nonce** | Option<**f32**> | Random nonce number received from last JoinReq message | [optional]
**last_join** | Option<[**String**](string.md)> | Date from last successful JoinReq | [optional]
**last_seen** | Option<**f32**> | Last device seen date | [optional]
**rssi** | Option<**f32**> | Received Signal Strength Indicator | [optional]
**snr** | Option<**f32**> | Signal Noise Ratio | [optional]
**freq** | Option<**f32**> | Frequency of last frame | [optional]
**sf** | Option<**f32**> | Spreading Factor of last frame | [optional]
**bw** | Option<**f32**> | Bandwidth of last frame | [optional]
**gw** | Option<**String**> | Gateway EUI of last frame received | [optional]
**appeui** | Option<**String**> | Global application ID in IEEE EUI64 address space that uniquely identifies the application provider (i.e., owner) of the end-device.  | [optional]
**last_dev_status_seen** | Option<[**String**](string.md)> | Date of last time DevStatus MAC command received | [optional]
**bat** | Option<**f32**> | Battery value encoded to a byte from DevStatusAns command | [optional]
**dev_snr** | Option<**f32**> | Device demulation signal-to-noise ratio in Db to the nearest integer value for the last successfully received DevStatusReq command  | [optional]
**lorawan** | Option<[**crate::models::LoriotDeviceLorawan**](Device_lorawan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


