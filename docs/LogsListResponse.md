# LogsListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**logs** | Option<[**Vec<crate::models::Log>**](Log.md)> | Array of logs matching the request and the `nextLogId` if sent. | [optional]
**next_log_id** | Option<**String**> | Hash identifier of the next log to return in the list. This parameter is used for the pagination feature. | [optional]
**status** | Option<**String**> | Status of the response. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


