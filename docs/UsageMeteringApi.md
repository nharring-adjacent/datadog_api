# \UsageMeteringApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_daily_custom_reports**](UsageMeteringApi.md#get_daily_custom_reports) | **get** /api/v1/daily_custom_reports | Get the list of available daily custom reports
[**get_incident_management**](UsageMeteringApi.md#get_incident_management) | **get** /api/v1/usage/incident-management | Get hourly usage for incident management
[**get_ingested_spans**](UsageMeteringApi.md#get_ingested_spans) | **get** /api/v1/usage/ingested-spans | Get hourly usage for ingested spans
[**get_monthly_custom_reports**](UsageMeteringApi.md#get_monthly_custom_reports) | **get** /api/v1/monthly_custom_reports | Get the list of available monthly custom reports
[**get_specified_daily_custom_reports**](UsageMeteringApi.md#get_specified_daily_custom_reports) | **get** /api/v1/daily_custom_reports/{report_id} | Get specified daily custom reports
[**get_specified_monthly_custom_reports**](UsageMeteringApi.md#get_specified_monthly_custom_reports) | **get** /api/v1/monthly_custom_reports/{report_id} | Get specified monthly custom reports
[**get_tracing_without_limits**](UsageMeteringApi.md#get_tracing_without_limits) | **get** /api/v1/usage/tracing-without-limits | Get hourly usage for tracing without limits
[**get_usage_analyzed_logs**](UsageMeteringApi.md#get_usage_analyzed_logs) | **get** /api/v1/usage/analyzed_logs | Get hourly usage for analyzed logs
[**get_usage_attribution**](UsageMeteringApi.md#get_usage_attribution) | **get** /api/v1/usage/attribution | Get Usage Attribution
[**get_usage_billable_summary**](UsageMeteringApi.md#get_usage_billable_summary) | **get** /api/v1/usage/billable-summary | Get billable usage across your account
[**get_usage_compliance_monitoring**](UsageMeteringApi.md#get_usage_compliance_monitoring) | **get** /api/v1/usage/compliance-monitoring | Get hourly usage for Compliance Monitoring
[**get_usage_fargate**](UsageMeteringApi.md#get_usage_fargate) | **get** /api/v1/usage/fargate | Get hourly usage for Fargate
[**get_usage_hosts**](UsageMeteringApi.md#get_usage_hosts) | **get** /api/v1/usage/hosts | Get hourly usage for hosts and containers
[**get_usage_indexed_spans**](UsageMeteringApi.md#get_usage_indexed_spans) | **get** /api/v1/usage/indexed-spans | Get hourly usage for indexed spans
[**get_usage_internet_of_things**](UsageMeteringApi.md#get_usage_internet_of_things) | **get** /api/v1/usage/iot | Get hourly usage for IoT
[**get_usage_lambda**](UsageMeteringApi.md#get_usage_lambda) | **get** /api/v1/usage/aws_lambda | Get hourly usage for Lambda
[**get_usage_logs**](UsageMeteringApi.md#get_usage_logs) | **get** /api/v1/usage/logs | Get hourly usage for Logs
[**get_usage_logs_by_index**](UsageMeteringApi.md#get_usage_logs_by_index) | **get** /api/v1/usage/logs_by_index | Get hourly usage for Logs by Index
[**get_usage_network_flows**](UsageMeteringApi.md#get_usage_network_flows) | **get** /api/v1/usage/network_flows | Get hourly usage for Network Flows
[**get_usage_network_hosts**](UsageMeteringApi.md#get_usage_network_hosts) | **get** /api/v1/usage/network_hosts | Get hourly usage for Network Hosts
[**get_usage_profiling**](UsageMeteringApi.md#get_usage_profiling) | **get** /api/v1/usage/profiling | Get hourly usage for profiled hosts
[**get_usage_rum_sessions**](UsageMeteringApi.md#get_usage_rum_sessions) | **get** /api/v1/usage/rum_sessions | Get hourly usage for RUM Sessions
[**get_usage_snmp**](UsageMeteringApi.md#get_usage_snmp) | **get** /api/v1/usage/snmp | Get hourly usage for SNMP devices
[**get_usage_summary**](UsageMeteringApi.md#get_usage_summary) | **get** /api/v1/usage/summary | Get usage across your multi-org account
[**get_usage_synthetics**](UsageMeteringApi.md#get_usage_synthetics) | **get** /api/v1/usage/synthetics | Get hourly usage for Synthetics Checks
[**get_usage_synthetics_api**](UsageMeteringApi.md#get_usage_synthetics_api) | **get** /api/v1/usage/synthetics_api | Get hourly usage for Synthetics API Checks
[**get_usage_synthetics_browser**](UsageMeteringApi.md#get_usage_synthetics_browser) | **get** /api/v1/usage/synthetics_browser | Get hourly usage for Synthetics Browser Checks
[**get_usage_timeseries**](UsageMeteringApi.md#get_usage_timeseries) | **get** /api/v1/usage/timeseries | Get hourly usage for custom metrics
[**get_usage_top_avg_metrics**](UsageMeteringApi.md#get_usage_top_avg_metrics) | **get** /api/v1/usage/top_avg_metrics | Get top custom metrics by hourly average
[**get_usage_trace**](UsageMeteringApi.md#get_usage_trace) | **get** /api/v1/usage/traces | Get hourly usage for Trace Search



## get_daily_custom_reports

> crate::models::UsageCustomReportsResponse get_daily_custom_reports(page_size, page_number, sort_dir, sort)
Get the list of available daily custom reports

Get daily custom reports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i64**> | The number of files to return in the response. `[default=60]`. |  |
**page_number** | Option<**i64**> | The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`. |  |
**sort_dir** | Option<[**crate::models::UsageSortDirection**](.md)> | The direction to sort by: `[desc, asc]`. |  |
**sort** | Option<[**crate::models::UsageSort**](.md)> | The field to sort by: `[computed_on, size, start_date, end_date]`. |  |

### Return type

[**crate::models::UsageCustomReportsResponse**](UsageCustomReportsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_incident_management

> crate::models::UsageIncidentManagementResponse get_incident_management(start_hr, end_hr)
Get hourly usage for incident management

Get hourly usage for incident management.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageIncidentManagementResponse**](UsageIncidentManagementResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ingested_spans

> crate::models::UsageIngestedSpansResponse get_ingested_spans(start_hr, end_hr)
Get hourly usage for ingested spans

Get hourly usage for ingested spans.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageIngestedSpansResponse**](UsageIngestedSpansResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_monthly_custom_reports

> crate::models::UsageCustomReportsResponse get_monthly_custom_reports(page_size, page_number, sort_dir, sort)
Get the list of available monthly custom reports

Get monthly custom reports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i64**> | The number of files to return in the response `[default=60].` |  |
**page_number** | Option<**i64**> | The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`. |  |
**sort_dir** | Option<[**crate::models::UsageSortDirection**](.md)> | The direction to sort by: `[desc, asc]`. |  |
**sort** | Option<[**crate::models::UsageSort**](.md)> | The field to sort by: `[computed_on, size, start_date, end_date]`. |  |

### Return type

[**crate::models::UsageCustomReportsResponse**](UsageCustomReportsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_specified_daily_custom_reports

> crate::models::UsageSpecifiedCustomReportsResponse get_specified_daily_custom_reports(report_id)
Get specified daily custom reports

Get specified daily custom reports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | The specified ID to search results for. | [required] |

### Return type

[**crate::models::UsageSpecifiedCustomReportsResponse**](UsageSpecifiedCustomReportsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_specified_monthly_custom_reports

> crate::models::UsageSpecifiedCustomReportsResponse get_specified_monthly_custom_reports(report_id)
Get specified monthly custom reports

Get specified monthly custom reports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | The specified ID to search results for. | [required] |

### Return type

[**crate::models::UsageSpecifiedCustomReportsResponse**](UsageSpecifiedCustomReportsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tracing_without_limits

> crate::models::UsageTracingWithoutLimitsResponse get_tracing_without_limits(start_hr, end_hr)
Get hourly usage for tracing without limits

Get hourly usage for tracing without limits.  **Note** This endpoint has been renamed to `/api/v1/usage/ingested-spans`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageTracingWithoutLimitsResponse**](UsageTracingWithoutLimitsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_analyzed_logs

> crate::models::UsageAnalyzedLogsResponse get_usage_analyzed_logs(start_hr, end_hr)
Get hourly usage for analyzed logs

Get hourly usage for analyzed logs (Security Monitoring).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageAnalyzedLogsResponse**](UsageAnalyzedLogsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_attribution

> crate::models::UsageAttributionResponse get_usage_attribution(start_month, fields, end_month, sort_direction, sort_name)
Get Usage Attribution

Get Usage Attribution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_month** | **String** | Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage beginning in this month. Maximum of 15 months ago. | [required] |
**fields** | **String** | The specified field to search results for. | [required] |
**end_month** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month. |  |
**sort_direction** | Option<[**crate::models::UsageSortDirection**](.md)> | The direction to sort by: `[desc, asc]`. |  |
**sort_name** | Option<[**crate::models::UsageAttributionSort**](.md)> | The field to sort by. |  |

### Return type

[**crate::models::UsageAttributionResponse**](UsageAttributionResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_billable_summary

> crate::models::UsageBillableSummaryResponse get_usage_billable_summary(month)
Get billable usage across your account

Get billable usage across your account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**month** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage starting this month. |  |

### Return type

[**crate::models::UsageBillableSummaryResponse**](UsageBillableSummaryResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_compliance_monitoring

> crate::models::UsageComplianceResponse get_usage_compliance_monitoring(start_hr, end_hr)
Get hourly usage for Compliance Monitoring

Get hourly usage for Compliance Monitoring.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageComplianceResponse**](UsageComplianceResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_fargate

> crate::models::UsageFargateResponse get_usage_fargate(start_hr, end_hr)
Get hourly usage for Fargate

Get hourly usage for [Fargate](https://docs.datadoghq.com/integrations/ecs_fargate/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageFargateResponse**](UsageFargateResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_hosts

> crate::models::UsageHostsResponse get_usage_hosts(start_hr, end_hr)
Get hourly usage for hosts and containers

Get hourly usage for hosts and containers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageHostsResponse**](UsageHostsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_indexed_spans

> crate::models::UsageIndexedSpansResponse get_usage_indexed_spans(start_hr, end_hr)
Get hourly usage for indexed spans

Get hourly usage for indexed spans.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageIndexedSpansResponse**](UsageIndexedSpansResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_internet_of_things

> crate::models::UsageIoTResponse get_usage_internet_of_things(start_hr, end_hr)
Get hourly usage for IoT

Get hourly usage for IoT.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageIoTResponse**](UsageIoTResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_lambda

> crate::models::UsageLambdaResponse get_usage_lambda(start_hr, end_hr)
Get hourly usage for Lambda

Get hourly usage for lambda.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageLambdaResponse**](UsageLambdaResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_logs

> crate::models::UsageLogsResponse get_usage_logs(start_hr, end_hr)
Get hourly usage for Logs

Get hourly usage for logs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageLogsResponse**](UsageLogsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_logs_by_index

> crate::models::UsageLogsByIndexResponse get_usage_logs_by_index(start_hr, end_hr, index_name)
Get hourly usage for Logs by Index

Get hourly usage for logs by index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |
**index_name** | Option<[**Vec<String>**](String.md)> | Comma-separated list of log index names. |  |

### Return type

[**crate::models::UsageLogsByIndexResponse**](UsageLogsByIndexResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_network_flows

> crate::models::UsageNetworkFlowsResponse get_usage_network_flows(start_hr, end_hr)
Get hourly usage for Network Flows

Get hourly usage for network flows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageNetworkFlowsResponse**](UsageNetworkFlowsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_network_hosts

> crate::models::UsageNetworkHostsResponse get_usage_network_hosts(start_hr, end_hr)
Get hourly usage for Network Hosts

Get hourly usage for network hosts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageNetworkHostsResponse**](UsageNetworkHostsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_profiling

> crate::models::UsageProfilingResponse get_usage_profiling(start_hr, end_hr)
Get hourly usage for profiled hosts

Get hourly usage for profiled hosts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageProfilingResponse**](UsageProfilingResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_rum_sessions

> crate::models::UsageRumSessionsResponse get_usage_rum_sessions(start_hr, end_hr, _type)
Get hourly usage for RUM Sessions

Get hourly usage for [RUM](https://docs.datadoghq.com/real_user_monitoring/) Sessions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |
**_type** | Option<**String**> | RUM type: `[browser, mobile]`. Defaults to `browser`. |  |

### Return type

[**crate::models::UsageRumSessionsResponse**](UsageRumSessionsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_snmp

> crate::models::UsageSnmpResponse get_usage_snmp(start_hr, end_hr)
Get hourly usage for SNMP devices

Get hourly usage for SNMP devices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageSnmpResponse**](UsageSNMPResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_summary

> crate::models::UsageSummaryResponse get_usage_summary(start_month, end_month, include_org_details)
Get usage across your multi-org account

Get usage across your multi-org account. You must have the multi-org feature enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_month** | **String** | Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage beginning in this month. Maximum of 15 months ago. | [required] |
**end_month** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month. |  |
**include_org_details** | Option<**bool**> | Include usage summaries for each sub-org. |  |

### Return type

[**crate::models::UsageSummaryResponse**](UsageSummaryResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_synthetics

> crate::models::UsageSyntheticsResponse get_usage_synthetics(start_hr, end_hr)
Get hourly usage for Synthetics Checks

Get hourly usage for [Synthetics checks](https://docs.datadoghq.com/synthetics/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageSyntheticsResponse**](UsageSyntheticsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_synthetics_api

> crate::models::UsageSyntheticsApiResponse get_usage_synthetics_api(start_hr, end_hr)
Get hourly usage for Synthetics API Checks

Get hourly usage for [synthetics API checks](https://docs.datadoghq.com/synthetics/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageSyntheticsApiResponse**](UsageSyntheticsAPIResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_synthetics_browser

> crate::models::UsageSyntheticsBrowserResponse get_usage_synthetics_browser(start_hr, end_hr)
Get hourly usage for Synthetics Browser Checks

Get hourly usage for synthetics browser checks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageSyntheticsBrowserResponse**](UsageSyntheticsBrowserResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_timeseries

> crate::models::UsageTimeseriesResponse get_usage_timeseries(start_hr, end_hr)
Get hourly usage for custom metrics

Get hourly usage for [custom metrics](https://docs.datadoghq.com/developers/metrics/custom_metrics/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageTimeseriesResponse**](UsageTimeseriesResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_top_avg_metrics

> crate::models::UsageTopAvgMetricsResponse get_usage_top_avg_metrics(month, day, names, limit)
Get top custom metrics by hourly average

Get top [custom metrics](https://docs.datadoghq.com/developers/metrics/custom_metrics/) by hourly average. Use the month parameter to get a month-to-date data resolution or use the day parameter to get a daily resolution. One of the two is required, and only one of the two is allowed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**month** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to month: [YYYY-MM] for usage beginning at this hour. (Either month or day should be specified, but not both) |  |
**day** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to day: [YYYY-MM-DD] for usage beginning at this hour. (Either month or day should be specified, but not both) |  |
**names** | Option<[**Vec<String>**](String.md)> | Comma-separated list of metric names. |  |
**limit** | Option<**i32**> | Maximum number of results to return (between 1 and 5000) - defaults to 500 results if limit not specified. |  |[default to 500]

### Return type

[**crate::models::UsageTopAvgMetricsResponse**](UsageTopAvgMetricsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_trace

> crate::models::UsageTraceResponse get_usage_trace(start_hr, end_hr)
Get hourly usage for Trace Search

Get hourly usage for trace search.  **Note** This endpoint has been renamed to `/api/v1/usage/indexed-spans`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_hr** | **String** | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. | [required] |
**end_hr** | Option<**String**> | Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. |  |

### Return type

[**crate::models::UsageTraceResponse**](UsageTraceResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;datetime-format=rfc3339

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

