# SyntheticsTestRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**basic_auth** | Option<[**crate::models::SyntheticsBasicAuth**](SyntheticsBasicAuth.md)> |  | [optional]
**body** | Option<**String**> | Body to include in the test. | [optional]
**certificate** | Option<[**crate::models::SyntheticsTestRequestCertificate**](SyntheticsTestRequestCertificate.md)> |  | [optional]
**dns_server** | Option<**String**> | DNS server to use for DNS tests. | [optional]
**dns_server_port** | Option<**i32**> | DNS server port to use for DNS tests. | [optional]
**headers** | Option<**::std::collections::HashMap<String, String>**> | Headers to include when performing the test. | [optional]
**host** | Option<**String**> | Host name to perform the test with. | [optional]
**method** | Option<[**crate::models::HttpMethod**](HTTPMethod.md)> |  | [optional]
**no_saving_response_body** | Option<**bool**> | Determines whether or not to save the response body. | [optional]
**number_of_packets** | Option<**i32**> | Number of pings to use per test. | [optional]
**port** | Option<**i64**> | Port to use when performing the test. | [optional]
**query** | Option<[**serde_json::Value**](.md)> | Query to use for the test. | [optional]
**should_track_hops** | Option<**bool**> | Turns on a traceroute probe to discover all gateways along the path to the host destination. | [optional]
**timeout** | Option<**f64**> | Timeout in seconds for the test. | [optional]
**url** | Option<**String**> | URL to perform the test with. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


