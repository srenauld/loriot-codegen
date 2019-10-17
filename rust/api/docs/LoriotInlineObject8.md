# LoriotInlineObject8

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | Device title, EUI by default | 
**description** | Option<**String**> | Device description, empty by default | [optional]
**devclass** | **String** | Device LoRaWAN class type, 'A' by default. | 
**appkey** | **String** | AES-128 application key specific for the end-device that is assigned by the application owner to the end-device and most likely derived from an application-specific root key exclusively known to and under the control of the application provider.  | 
**deveui** | **String** | Required for OTAA register or to enroll new device. DevEUI is a global end-device ID in IEEE EUI64 address space that uniquely identifies the end-device.  | 
**appeui** | **String** | AppEUI is a global application ID in IEEE EUI64 address space that uniquely identifies the application provider (i.e., owner) of the end-device.  | 
**nwkskey** | Option<**String**> | NwkSKey is a network session key specific for the end-device. It is used by both the network server and the end-device to calculate and verify the MIC (message integrity code) of all  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


