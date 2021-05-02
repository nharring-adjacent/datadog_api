# SyntheticsCiTest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_insecure_certificates** | Option<**bool**> | Disable certificate checks in API tests. | [optional]
**basic_auth** | Option<[**crate::models::SyntheticsBasicAuth**](SyntheticsBasicAuth.md)> |  | [optional]
**body** | Option<**String**> | Body to include in the test. | [optional]
**body_type** | Option<**String**> | Type of the data sent in a synthetics API test. | [optional]
**cookies** | Option<**String**> | Cookies for the request. | [optional]
**device_ids** | Option<[**Vec<crate::models::SyntheticsDeviceId>**](SyntheticsDeviceID.md)> | For browser test, array with the different device IDs used to run the test. | [optional]
**follow_redirects** | Option<**bool**> | For API HTTP test, whether or not the test should follow redirects. | [optional]
**headers** | Option<**::std::collections::HashMap<String, String>**> | Headers to include when performing the test. | [optional]
**locations** | Option<**Vec<String>**> | Array of locations used to run the test. | [optional]
**metadata** | Option<[**crate::models::SyntheticsCiTestMetadata**](SyntheticsCITest_metadata.md)> |  | [optional]
**public_id** | **String** | The public ID of the Synthetics test to trigger. | 
**retry** | Option<[**crate::models::SyntheticsTestOptionsRetry**](SyntheticsTestOptionsRetry.md)> |  | [optional]
**start_url** | Option<**String**> | Starting URL for the browser test. | [optional]
**variables** | Option<**::std::collections::HashMap<String, String>**> | Variables to replace in the test. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


