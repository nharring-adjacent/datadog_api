# MetricsQueryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | Option<**String**> | Message indicating the errors if status is not `ok`. | [optional][readonly]
**from_date** | Option<**i64**> | Start of requested time window, milliseconds since Unix epoch. | [optional][readonly]
**group_by** | Option<**Vec<String>**> | List of tag keys on which to group. | [optional][readonly]
**message** | Option<**String**> | Message indicating `success` if status is `ok`. | [optional][readonly]
**query** | Option<**String**> | Query string | [optional][readonly]
**res_type** | Option<**String**> | Type of response. | [optional][readonly]
**series** | Option<[**Vec<crate::models::MetricsQueryMetadata>**](MetricsQueryMetadata.md)> | List of timeseries queried. | [optional][readonly]
**status** | Option<**String**> | Status of the query. | [optional][readonly]
**to_date** | Option<**i64**> | End of requested time window, milliseconds since Unix epoch. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


