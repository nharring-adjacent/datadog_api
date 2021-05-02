# LogContent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attributes** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | JSON object of attributes from your log. | [optional]
**host** | Option<**String**> | Name of the machine from where the logs are being sent. | [optional]
**message** | Option<**String**> | The message [reserved attribute](https://docs.datadoghq.com/logs/log_collection/#reserved-attributes) of your log. By default, Datadog ingests the value of the message attribute as the body of the log entry. That value is then highlighted and displayed in the Logstream, where it is indexed for full text search. | [optional]
**service** | Option<**String**> | The name of the application or service generating the log events. It is used to switch from Logs to APM, so make sure you define the same value when you use both products. | [optional]
**tags** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array of tags associated with your log. | [optional]
**timestamp** | Option<**String**> | Timestamp of your log. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


