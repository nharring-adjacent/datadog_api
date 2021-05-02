# SyntheticsTestOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accept_self_signed** | Option<**bool**> | For SSL test, whether or not the test should allow self signed certificates. | [optional]
**allow_insecure** | Option<**bool**> | Allows loading insecure content for an HTTP request. | [optional]
**device_ids** | Option<[**Vec<crate::models::SyntheticsDeviceId>**](SyntheticsDeviceID.md)> | For browser test, array with the different device IDs used to run the test. | [optional]
**disable_cors** | Option<**bool**> | Whether or not to disable CORS mechanism. Currently only available for Chrome. | [optional]
**follow_redirects** | Option<**bool**> | For API HTTP test, whether or not the test should follow redirects. | [optional]
**min_failure_duration** | Option<**i64**> | Minimum amount of time in failure required to trigger an alert. | [optional]
**min_location_failed** | Option<**i64**> | Minimum number of locations in failure required to trigger an alert. | [optional]
**monitor_options** | Option<[**crate::models::SyntheticsTestOptionsMonitorOptions**](SyntheticsTestOptions_monitor_options.md)> |  | [optional]
**no_screenshot** | Option<**bool**> | Prevents saving screenshots of the steps. | [optional]
**retry** | Option<[**crate::models::SyntheticsTestOptionsRetry**](SyntheticsTestOptionsRetry.md)> |  | [optional]
**tick_every** | Option<[**crate::models::SyntheticsTickInterval**](SyntheticsTickInterval.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


