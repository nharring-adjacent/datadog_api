# Host

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aliases** | Option<**Vec<String>**> | Host aliases collected by Datadog. | [optional]
**apps** | Option<**Vec<String>**> | The Datadog integrations reporting metrics for the host. | [optional]
**aws_name** | Option<**String**> | AWS name of your host. | [optional]
**host_name** | Option<**String**> | The host name. | [optional]
**id** | Option<**i64**> | The host ID. | [optional]
**is_muted** | Option<**bool**> | If a host is muted or unmuted. | [optional]
**last_reported_time** | Option<**i64**> | Last time the host reported a metric data point. | [optional]
**meta** | Option<[**crate::models::HostMeta**](Host_meta.md)> |  | [optional]
**metrics** | Option<[**crate::models::HostMetrics**](Host_metrics.md)> |  | [optional]
**mute_timeout** | Option<**i64**> | Timeout of the mute applied to your host. | [optional]
**name** | Option<**String**> | The host name. | [optional]
**sources** | Option<**Vec<String>**> | Source or cloud provider associated with your host. | [optional]
**tags_by_source** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> | List of tags for each source (AWS, Datadog Agent, Chef..). | [optional]
**up** | Option<**bool**> | Displays UP when the expected metrics are received and displays `???` if no metrics are received. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


