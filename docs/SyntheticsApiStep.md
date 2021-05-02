# SyntheticsApiStep

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_failure** | Option<**bool**> | Determines whether or not to continue with test if this step fails. | [optional]
**assertions** | Option<[**Vec<crate::models::SyntheticsAssertion>**](SyntheticsAssertion.md)> | Array of assertions used for the test. | [optional][default to []]
**extracted_values** | Option<[**Vec<crate::models::SyntheticsParsingOptions>**](SyntheticsParsingOptions.md)> | Array of values to parse and save as variables from the response. | [optional]
**is_critical** | Option<**bool**> | Determines whether or not to consider the entire test as failed if this step fails. Can be used only if `allowFailure` is `true`. | [optional]
**name** | Option<**String**> | The name of the step. | [optional]
**request** | Option<[**crate::models::SyntheticsTestRequest**](SyntheticsTestRequest.md)> |  | [optional]
**subtype** | Option<[**crate::models::SyntheticsApiStepSubtype**](SyntheticsAPIStepSubtype.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


