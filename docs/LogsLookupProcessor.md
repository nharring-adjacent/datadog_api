# LogsLookupProcessor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_lookup** | Option<**String**> | Value to set the target attribute if the source value is not found in the list. | [optional]
**is_enabled** | Option<**bool**> | Whether or not the processor is enabled. | [optional][default to false]
**lookup_table** | **Vec<String>** | Mapping table of values for the source attribute and their associated target attribute values, formatted as `[\"source_key1,target_value1\", \"source_key2,target_value2\"]` | 
**name** | Option<**String**> | Name of the processor. | [optional]
**source** | **String** | Source attribute used to perform the lookup. | 
**target** | **String** | Name of the attribute that contains the corresponding value in the mapping list or the `default_lookup` if not found in the mapping list. | 
**_type** | [**crate::models::LogsLookupProcessorType**](LogsLookupProcessorType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


