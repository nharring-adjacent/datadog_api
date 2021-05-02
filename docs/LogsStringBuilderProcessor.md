# LogsStringBuilderProcessor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_enabled** | Option<**bool**> | Whether or not the processor is enabled. | [optional][default to false]
**is_replace_missing** | Option<**bool**> | If true, it replaces all missing attributes of `template` by an empty string. If `false` (default), skips the operation for missing attributes. | [optional][default to false]
**name** | Option<**String**> | Name of the processor. | [optional]
**target** | **String** | The name of the attribute that contains the result of the template. | 
**template** | **String** | A formula with one or more attributes and raw text. | 
**_type** | [**crate::models::LogsStringBuilderProcessorType**](LogsStringBuilderProcessorType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


