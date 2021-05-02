# EventCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregation_key** | Option<**String**> | An arbitrary string to use for aggregation. Limited to 100 characters. If you specify a key, all events using that key are grouped together in the Event Stream. | [optional]
**alert_type** | Option<[**crate::models::EventAlertType**](EventAlertType.md)> |  | [optional]
**date_happened** | Option<**i64**> | POSIX timestamp of the event. Must be sent as an integer (i.e. no quotes). Limited to events no older than 7 days. | [optional]
**device_name** | Option<**String**> | A device name. | [optional]
**host** | Option<**String**> | Host name to associate with the event. Any tags associated with the host are also applied to this event. | [optional]
**id** | Option<**i64**> | Integer ID of the event. | [optional][readonly]
**payload** | Option<**String**> | Payload of the event. | [optional][readonly]
**priority** | Option<[**crate::models::EventPriority**](EventPriority.md)> |  | [optional]
**related_event_id** | Option<**i64**> | ID of the parent event. Must be sent as an integer (i.e. no quotes). | [optional]
**source_type_name** | Option<**String**> | The type of event being posted. Option examples include nagios, hudson, jenkins, my_apps, chef, puppet, git, bitbucket, etc. A complete list of source attribute values [available here](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value). | [optional]
**tags** | Option<**Vec<String>**> | A list of tags to apply to the event. | [optional]
**text** | **String** | The body of the event. Limited to 4000 characters. The text supports markdown. To use markdown in the event text, start the text block with `%%% \\n` and end the text block with `\\n %%%`. Use `msg_text` with the Datadog Ruby library. | 
**title** | **String** | The event title. Limited to 100 characters. Use `msg_title` with the Datadog Ruby library. | 
**url** | Option<**String**> | URL of the event. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


