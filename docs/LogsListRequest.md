# LogsListRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | Option<**String**> | The log index on which the request is performed. For multi-index organizations, the default is all live indexes. Historical indexes of rehydrated logs must be specified. | [optional]
**limit** | Option<**i32**> | Number of logs return in the response. | [optional]
**query** | Option<**String**> | The search query - following the log search syntax. | [optional]
**sort** | Option<[**crate::models::LogsSort**](LogsSort.md)> |  | [optional]
**start_at** | Option<**String**> | Hash identifier of the first log to return in the list, available in a log `id` attribute. This parameter is used for the pagination feature.  **Note**: This parameter is ignored if the corresponding log is out of the scope of the specified time window. | [optional]
**time** | [**crate::models::LogsListRequestTime**](LogsListRequest_time.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


