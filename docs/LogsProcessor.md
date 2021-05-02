# LogsProcessor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**grok** | [**crate::models::LogsGrokParserRules**](LogsGrokParserRules.md) |  | 
**is_enabled** | Option<**bool**> | Whether or not the processor is enabled. | [optional][default to false]
**name** | Option<**String**> | Name of the processor. | [optional]
**samples** | Option<**Vec<String>**> | List of sample logs to test this grok parser. | [optional]
**source** | **String** | Source attribute used to perform the lookup. | 
**_type** | [**crate::models::LogsTraceRemapperType**](LogsTraceRemapperType.md) |  | 
**sources** | **Vec<String>** | Array of source attributes. | [default to ["dd.trace_id"]]
**override_on_conflict** | Option<**bool**> | Override or not the target element if already set, | [optional][default to false]
**preserve_source** | Option<**bool**> | Remove or preserve the remapped source element. | [optional][default to false]
**source_type** | Option<**String**> | Defines if the sources are from log `attribute` or `tag`. | [optional][default to attribute]
**target** | **String** | Name of the attribute that contains the corresponding value in the mapping list or the `default_lookup` if not found in the mapping list. | 
**target_format** | Option<[**crate::models::TargetFormatType**](TargetFormatType.md)> |  | [optional]
**target_type** | Option<**String**> | Defines if the final attribute or tag name is from log `attribute` or `tag`. | [optional][default to attribute]
**normalize_ending_slashes** | Option<**bool**> | Normalize the ending slashes or not. | [optional][default to false]
**is_encoded** | Option<**bool**> | Define if the source attribute is URL encoded or not. | [optional][default to false]
**categories** | [**Vec<crate::models::LogsCategoryProcessorCategory>**](LogsCategoryProcessorCategory.md) | Array of filters to match or not a log and their corresponding `name`to assign a custom value to the log. | 
**expression** | **String** | Arithmetic operation between one or more log attributes. | 
**is_replace_missing** | Option<**bool**> | If true, it replaces all missing attributes of `template` by an empty string. If `false` (default), skips the operation for missing attributes. | [optional][default to false]
**template** | **String** | A formula with one or more attributes and raw text. | 
**filter** | Option<[**crate::models::LogsFilter**](LogsFilter.md)> |  | [optional]
**processors** | Option<[**Vec<crate::models::LogsProcessor>**](LogsProcessor.md)> | Ordered list of processors in this pipeline. | [optional]
**default_lookup** | Option<**String**> | Value to set the target attribute if the source value is not found in the list. | [optional]
**lookup_table** | **Vec<String>** | Mapping table of values for the source attribute and their associated target attribute values, formatted as `[\"source_key1,target_value1\", \"source_key2,target_value2\"]` | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


