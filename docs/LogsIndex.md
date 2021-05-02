# LogsIndex

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**daily_limit** | Option<**i64**> | The number of log events you can send in this index per day before you are rate-limited. | [optional]
**exclusion_filters** | Option<[**Vec<crate::models::LogsExclusion>**](LogsExclusion.md)> | An array of exclusion objects. The logs are tested against the query of each filter, following the order of the array. Only the first matching active exclusion matters, others (if any) are ignored. | [optional]
**filter** | [**crate::models::LogsFilter**](LogsFilter.md) |  | 
**is_rate_limited** | Option<**bool**> | A boolean stating if the index is rate limited, meaning more logs than the daily limit have been sent. Rate limit is reset every-day at 2pm UTC. | [optional][readonly]
**name** | **String** | The name of the index. | 
**num_retention_days** | Option<**i64**> | The number of days before logs are deleted from this index. Available values depend on retention plans specified in your organization's contract/subscriptions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


