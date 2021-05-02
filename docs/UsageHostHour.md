# UsageHostHour

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent_host_count** | Option<**i64**> | Contains the total number of infrastructure hosts reporting during a given hour that were running the Datadog Agent. | [optional]
**alibaba_host_count** | Option<**i64**> | Contains the total number of hosts that reported via Alibaba integration (and were NOT running the Datadog Agent). | [optional]
**apm_azure_app_service_host_count** | Option<**i64**> | Contains the total number of Azure App Services hosts using APM. | [optional]
**apm_host_count** | Option<**i64**> | Shows the total number of hosts using APM during the hour, these are counted as billable (except during trial periods). | [optional]
**aws_host_count** | Option<**i64**> | Contains the total number of hosts that reported via the AWS integration (and were NOT running the Datadog Agent). | [optional]
**azure_host_count** | Option<**i64**> | Contains the total number of hosts that reported via Azure integration (and were NOT running the Datadog Agent). | [optional]
**container_count** | Option<**i64**> | Shows the total number of containers reported by the Docker integration during the hour. | [optional]
**gcp_host_count** | Option<**i64**> | Contains the total number of hosts that reported via the Google Cloud integration (and were NOT running the Datadog Agent). | [optional]
**heroku_host_count** | Option<**i64**> | Contains the total number of Heroku dynos reported by the Datadog Agent. | [optional]
**host_count** | Option<**i64**> | Contains the total number of billable infrastructure hosts reporting during a given hour. This is the sum of `agent_host_count`, `aws_host_count`, and `gcp_host_count`. | [optional]
**hour** | Option<**String**> | The hour for the usage. | [optional]
**infra_azure_app_service** | Option<**i64**> | Contains the total number of hosts that reported via the Azure App Services integration (and were NOT running the Datadog Agent). | [optional]
**opentelemetry_host_count** | Option<**i64**> | Contains the total number of hosts reported by Datadog exporter for the OpenTelemetry Collector. | [optional]
**vsphere_host_count** | Option<**i64**> | Contains the total number of hosts that reported via vSphere integration (and were NOT running the Datadog Agent). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


