# LogsExclusionFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | Option<**String**> | Default query is `*`, meaning all logs flowing in the index would be excluded. Scope down exclusion filter to only a subset of logs with a log query. | [optional]
**sample_rate** | **f64** | Sample rate to apply to logs going through this exclusion filter, a value of 1 will exclude all logs matching the query. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


