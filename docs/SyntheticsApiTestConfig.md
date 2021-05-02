# SyntheticsApiTestConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assertions** | [**Vec<crate::models::SyntheticsAssertion>**](SyntheticsAssertion.md) | Array of assertions used for the test. | [default to []]
**config_variables** | Option<[**Vec<crate::models::SyntheticsConfigVariable>**](SyntheticsConfigVariable.md)> | Array of variables used for the test. | [optional]
**request** | Option<[**crate::models::SyntheticsTestRequest**](SyntheticsTestRequest.md)> |  | [optional]
**steps** | Option<[**Vec<crate::models::SyntheticsApiStep>**](SyntheticsAPIStep.md)> | When the test subtype is `multi`, the steps of the test. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


