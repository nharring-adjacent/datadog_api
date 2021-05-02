# LogsUserAgentParser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_enabled** | Option<**bool**> | Whether or not the processor is enabled. | [optional][default to false]
**is_encoded** | Option<**bool**> | Define if the source attribute is URL encoded or not. | [optional][default to false]
**name** | Option<**String**> | Name of the processor. | [optional]
**sources** | **Vec<String>** | Array of source attributes. | [default to ["http.useragent"]]
**target** | **String** | Name of the parent attribute that contains all the extracted details from the `sources`. | [default to http.useragent_details]
**_type** | [**crate::models::LogsUserAgentParserType**](LogsUserAgentParserType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


