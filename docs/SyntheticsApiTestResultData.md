# SyntheticsApiTestResultData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cert** | Option<[**crate::models::SyntheticsSslCertificate**](SyntheticsSSLCertificate.md)> |  | [optional]
**error_code** | Option<[**crate::models::SyntheticsErrorCode**](SyntheticsErrorCode.md)> |  | [optional]
**error_message** | Option<**String**> | The API test error message. | [optional]
**event_type** | Option<[**crate::models::SyntheticsTestProcessStatus**](SyntheticsTestProcessStatus.md)> |  | [optional]
**http_status_code** | Option<**i64**> | The API test HTTP status code. | [optional]
**request_headers** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Request header object used for the API test. | [optional]
**response_body** | Option<**String**> | Response body returned for the API test. | [optional]
**response_headers** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Response headers returned for the API test. | [optional]
**response_size** | Option<**i64**> | Global size in byte of the API test response. | [optional]
**timings** | Option<[**crate::models::SyntheticsTiming**](SyntheticsTiming.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


