# LogsIndexUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**daily_limit** | Option<**i64**> | The number of log events you can send in this index per day before you are rate-limited. | [optional]
**disable_daily_limit** | Option<**bool**> | If true, sets the `daily_limit` value to null and the index is not limited on a daily basis (any specified `daily_limit` value in the request is ignored). If false or omitted, the index's current `daily_limit` is maintained. | [optional]
**exclusion_filters** | Option<[**Vec<crate::models::LogsExclusion>**](LogsExclusion.md)> | An array of exclusion objects. The logs are tested against the query of each filter, following the order of the array. Only the first matching active exclusion matters, others (if any) are ignored. | [optional]
**filter** | [**crate::models::LogsFilter**](LogsFilter.md) |  | 
**num_retention_days** | Option<**i64**> | The number of days before logs are deleted from this index. Available values depend on retention plans specified in your organization's contract/subscriptions.  **Note:** Changing the retention for an index adjusts the length of retention for all logs already in this index. It may also affect billing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


