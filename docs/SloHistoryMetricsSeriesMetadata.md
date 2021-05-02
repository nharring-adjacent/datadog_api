# SloHistoryMetricsSeriesMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggr** | Option<**String**> | Query aggregator function. | [optional]
**expression** | Option<**String**> | Query expression. | [optional]
**metric** | Option<**String**> | Query metric used. | [optional]
**query_index** | Option<**i64**> | Query index from original combined query. | [optional]
**scope** | Option<**String**> | Query scope. | [optional]
**unit** | Option<[**Vec<crate::models::SloHistoryMetricsSeriesMetadataUnit>**](SLOHistoryMetricsSeriesMetadataUnit.md)> | An array of metric units that contains up to two unit objects. For example, bytes represents one unit object and bytes per second represents two unit objects. If a metric query only has one unit object, the second array element is null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


