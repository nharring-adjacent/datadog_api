# \HostsApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_host_totals**](HostsApi.md#get_host_totals) | **get** /api/v1/hosts/totals | Get the total number of active hosts
[**list_hosts**](HostsApi.md#list_hosts) | **get** /api/v1/hosts | Get all hosts for your organization
[**mute_host**](HostsApi.md#mute_host) | **post** /api/v1/host/{host_name}/mute | Mute a host
[**unmute_host**](HostsApi.md#unmute_host) | **post** /api/v1/host/{host_name}/unmute | Unmute a host



## get_host_totals

> crate::models::HostTotals get_host_totals(from)
Get the total number of active hosts

This endpoint returns the total number of active and up hosts in your Datadog account. Active means the host has reported in the past hour, and up means it has reported in the past two hours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**i64**> | Number of seconds from which you want to get total number of active hosts. |  |

### Return type

[**crate::models::HostTotals**](HostTotals.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_hosts

> crate::models::HostListResponse list_hosts(filter, sort_field, sort_dir, start, count, from, include_muted_hosts_data, include_hosts_metadata)
Get all hosts for your organization

This endpoint allows searching for hosts by name, alias, or tag. Hosts live within the past 3 hours are included by default. Retention is 7 days. Results are paginated with a max of 1000 results at a time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | String to filter search results. |  |
**sort_field** | Option<**String**> | Sort hosts by this field. |  |
**sort_dir** | Option<**String**> | Direction of sort. Options include `asc` and `desc`. |  |
**start** | Option<**i64**> | Host result to start search from. |  |
**count** | Option<**i64**> | Number of hosts to return. Max 1000. |  |
**from** | Option<**i64**> | Number of seconds since UNIX epoch from which you want to search your hosts. |  |
**include_muted_hosts_data** | Option<**bool**> | Include information on the muted status of hosts and when the mute expires. |  |
**include_hosts_metadata** | Option<**bool**> | Include additional metadata about the hosts (agent_version, machine, platform, processor, etc.). |  |

### Return type

[**crate::models::HostListResponse**](HostListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mute_host

> crate::models::HostMuteResponse mute_host(host_name, body)
Mute a host

Mute a host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_name** | **String** | Name of the host to mute. | [required] |
**body** | [**HostMuteSettings**](HostMuteSettings.md) | Mute a host request body. | [required] |

### Return type

[**crate::models::HostMuteResponse**](HostMuteResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unmute_host

> crate::models::HostMuteResponse unmute_host(host_name)
Unmute a host

Unmutes a host. This endpoint takes no JSON arguments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_name** | **String** | Name of the host to unmute. | [required] |

### Return type

[**crate::models::HostMuteResponse**](HostMuteResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

