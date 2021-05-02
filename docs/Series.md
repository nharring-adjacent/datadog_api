# Series

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | Option<**String**> | The name of the host that produced the metric. | [optional]
**interval** | Option<**i64**> | If the type of the metric is rate or count, define the corresponding interval. | [optional]
**metric** | **String** | The name of the timeseries. | 
**points** | [**Vec<crate::models::Array>**](array.md) | Points relating to a metric. All points must be tuples with timestamp and a scalar value (cannot be a string). Timestamps should be in POSIX time in seconds, and cannot be more than ten minutes in the future or more than one hour in the past. | 
**tags** | Option<**Vec<String>**> | A list of tags associated with the metric. | [optional]
**_type** | Option<**String**> | The type of the metric either `count`, `gauge`, or `rate`. | [optional][default to gauge]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


