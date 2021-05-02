# LogsAttributeRemapper

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_enabled** | Option<**bool**> | Whether or not the processor is enabled. | [optional][default to false]
**name** | Option<**String**> | Name of the processor. | [optional]
**override_on_conflict** | Option<**bool**> | Override or not the target element if already set, | [optional][default to false]
**preserve_source** | Option<**bool**> | Remove or preserve the remapped source element. | [optional][default to false]
**source_type** | Option<**String**> | Defines if the sources are from log `attribute` or `tag`. | [optional][default to attribute]
**sources** | **Vec<String>** | Array of source attributes. | 
**target** | **String** | Final attribute or tag name to remap the sources to. | 
**target_format** | Option<[**crate::models::TargetFormatType**](TargetFormatType.md)> |  | [optional]
**target_type** | Option<**String**> | Defines if the final attribute or tag name is from log `attribute` or `tag`. | [optional][default to attribute]
**_type** | [**crate::models::LogsAttributeRemapperType**](LogsAttributeRemapperType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


