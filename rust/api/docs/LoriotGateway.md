# LoriotGateway

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | Option<**String**> | Gateway EUI ID | [optional]
**MAC** | Option<**String**> | Gateway MAC | [optional]
**EUI** | Option<**String**> | Gateway EUI | [optional]
**base** | Option<**String**> | base / manufacturer of the gateway | [optional]
**model** | Option<**String**> | model of the gateway | [optional]
**location** | Option<[**crate::models::LoriotLocation**](Location.md)> |  | [optional]
**connected** | Option<**bool**> | Gateway connection status | [optional]
**radioband** | Option<**String**> | gateway channel ID | [optional]
**bus** | Option<**String**> | bus used to connect the concentrator | [optional]
**concentrator** | Option<**String**> | model of the concentrator | [optional]
**card** | Option<**f32**> | ID of the concentrator / bus / card | [optional]
**last_data** | Option<**String**> | date and time of last data | [optional]
**last_started** | Option<**String**> | Date of last gateway start of the binary / full reload | [optional]
**last_pong** | Option<**String**> | last keep-alive timestamp | [optional]
**version** | Option<**String**> | version reported by the gateway | [optional]
**rtt** | Option<**f32**> | latency (round trip time) between Network Server and Gateway from WebSocket protocol  | [optional]
**title** | Option<**String**> | custom gateway title updated by user | [optional]
**basename** | Option<**String**> | user friendly version of base | [optional]
**modelname** | Option<**String**> | user friendly version of model | [optional]
**concentratorname** | Option<**String**> | user friendly version of concentrator | [optional]
**alerts** | Option<**bool**> | flag enable / disable alerts for this gateway | [optional]
**created_at** | Option<**String**> | Creation date | [optional]
**visibility** | Option<**String**> | visibility of the gateway. Organization wide or only for the user | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


