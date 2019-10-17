# LoriotInlineObject7

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | Device title, EUI by default | 
**description** | Option<**String**> | Device description, empty by default | [optional]
**deveui** | **String** | DevEUI is a global end-device ID in IEEE EUI64 address space that uniquely identifies the end-device.  | 
**nwkskey** | **String** | NwkSKey is a network session key specific for the end-device. It is used by both the network server and the end-device to calculate and verify the MIC (message integrity code) of all data messages to ensure data integrity.  | 
**devaddr** | **String** | DevAddr consists of 32 bits identifies the end-device within the current network.  | 
**seqno** | **f32** | Uplink sequence number to initialize the device.  | 
**seqdn** | **f32** | Downlink sequence number to initialize the device.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


