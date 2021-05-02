# MetricsQueryMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggr** | Option<**String**> | Aggregation type. | [optional][readonly]
**display_name** | Option<**String**> | Display name of the metric. | [optional][readonly]
**end** | Option<**i64**> | End of the time window, milliseconds since Unix epoch. | [optional][readonly]
**expression** | Option<**String**> | Metric expression. | [optional][readonly]
**interval** | Option<**i64**> | Number of seconds between data samples. | [optional][readonly]
**length** | Option<**i64**> | Number of data samples. | [optional][readonly]
**metric** | Option<**String**> | Metric name. | [optional][readonly]
**pointlist** | Option<[**Vec<crate::models::Array>**](array.md)> | List of points of the time series. | [optional][readonly]
**scope** | Option<**String**> | Metric scope, comma separated list of tags. | [optional][readonly]
**start** | Option<**i64**> | Start of the time window, milliseconds since Unix epoch. | [optional][readonly]
**unit** | Option<[**Vec<crate::models::MetricsQueryUnit>**](MetricsQueryUnit.md)> | Detailed information about the metric unit. First element describes the \"primary unit\" (for example, `bytes` in `bytes per second`), second describes the \"per unit\" (for example, `second` in `bytes per second`). | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


