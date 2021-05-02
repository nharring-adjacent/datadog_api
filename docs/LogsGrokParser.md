# LogsGrokParser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**grok** | [**crate::models::LogsGrokParserRules**](LogsGrokParserRules.md) |  | 
**is_enabled** | Option<**bool**> | Whether or not the processor is enabled. | [optional][default to false]
**name** | Option<**String**> | Name of the processor. | [optional]
**samples** | Option<**Vec<String>**> | List of sample logs to test this grok parser. | [optional]
**source** | **String** | Name of the log attribute to parse. | [default to message]
**_type** | [**crate::models::LogsGrokParserType**](LogsGrokParserType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


