# MetricMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Metric description. | [optional]
**integration** | Option<**String**> | Name of the integration that sent the metric if applicable. | [optional][readonly]
**per_unit** | Option<**String**> | Per unit of the metric such as `second` in `bytes per second`. | [optional]
**short_name** | Option<**String**> | A more human-readable and abbreviated version of the metric name. | [optional]
**statsd_interval** | Option<**i64**> | StatsD flush interval of the metric in seconds if applicable. | [optional]
**_type** | Option<**String**> | Metric type such as `gauge` or `rate`. | [optional]
**unit** | Option<**String**> | Primary unit of the metric such as `byte` or `operation`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


