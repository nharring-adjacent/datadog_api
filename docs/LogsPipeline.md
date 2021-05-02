# LogsPipeline

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filter** | Option<[**crate::models::LogsFilter**](LogsFilter.md)> |  | [optional]
**id** | Option<**String**> | ID of the pipeline. | [optional][readonly]
**is_enabled** | Option<**bool**> | Whether or not the pipeline is enabled. | [optional]
**is_read_only** | Option<**bool**> | Whether or not the pipeline can be edited. | [optional][readonly]
**name** | **String** | Name of the pipeline. | 
**processors** | Option<[**Vec<crate::models::LogsProcessor>**](LogsProcessor.md)> | Ordered list of processors in this pipeline. | [optional]
**_type** | Option<**String**> | Type of pipeline. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


