# SyntheticsTiming

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dns** | Option<**f64**> | The duration in millisecond of the DNS lookup. | [optional]
**download** | Option<**f64**> | The time in millisecond to download the response. | [optional]
**first_byte** | Option<**f64**> | The time in millisecond to first byte. | [optional]
**handshake** | Option<**f64**> | The duration in millisecond of the TLS handshake. | [optional]
**redirect** | Option<**f64**> | The time in millisecond spent during redirections. | [optional]
**ssl** | Option<**f64**> | The duration in millisecond of the TLS handshake. | [optional]
**tcp** | Option<**f64**> | Time in millisecond to establish the TCP connection. | [optional]
**total** | Option<**f64**> | The overall time in millisecond the request took to be processed. | [optional]
**wait** | Option<**f64**> | Time spent in millisecond waiting for a response. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


