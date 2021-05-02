# LogsArithmeticProcessor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expression** | **String** | Arithmetic operation between one or more log attributes. | 
**is_enabled** | Option<**bool**> | Whether or not the processor is enabled. | [optional][default to false]
**is_replace_missing** | Option<**bool**> | If `true`, it replaces all missing attributes of expression by `0`, `false` skip the operation if an attribute is missing. | [optional][default to false]
**name** | Option<**String**> | Name of the processor. | [optional]
**target** | **String** | Name of the attribute that contains the result of the arithmetic operation. | 
**_type** | [**crate::models::LogsArithmeticProcessorType**](LogsArithmeticProcessorType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


