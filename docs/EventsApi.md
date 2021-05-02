# \EventsApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_event**](EventsApi.md#create_event) | **post** /api/v1/events | Post an event
[**get_event**](EventsApi.md#get_event) | **get** /api/v1/events/{event_id} | Get an event
[**list_events**](EventsApi.md#list_events) | **get** /api/v1/events | Query the event stream



## create_event

> crate::models::EventCreateResponse create_event(body)
Post an event

This endpoint allows you to post events to the stream. Tag them, set priority and event aggregate them with other events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EventCreateRequest**](EventCreateRequest.md) | Event request object | [required] |

### Return type

[**crate::models::EventCreateResponse**](EventCreateResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event

> crate::models::EventResponse get_event(event_id)
Get an event

This endpoint allows you to query for event details.  **Note**: If the event you’re querying contains markdown formatting of any kind, you may see characters such as `%`,`\\`,`n` in your output.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **i64** | The ID of the event. | [required] |

### Return type

[**crate::models::EventResponse**](EventResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_events

> crate::models::EventListResponse list_events(start, end, priority, sources, tags, unaggregated, exclude_aggregate, page)
Query the event stream

The event stream can be queried and filtered by time, priority, sources and tags.  **Notes**: - If the event you’re querying contains markdown formatting of any kind, you may see characters such as `%`,`\\`,`n` in your output.  - This endpoint returns a maximum of `1000` most recent results. To return additional results, identify the last timestamp of the last result and set that as the `end` query time to paginate the results. You can also use the page parameter to specify which set of `1000` results to return.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **i64** | POSIX timestamp. | [required] |
**end** | **i64** | POSIX timestamp. | [required] |
**priority** | Option<[**crate::models::EventPriority**](.md)> | Priority of your events, either `low` or `normal`. |  |
**sources** | Option<**String**> | A comma separated string of sources. |  |
**tags** | Option<**String**> | A comma separated list indicating what tags, if any, should be used to filter the list of monitors by scope. |  |
**unaggregated** | Option<**bool**> | Set unaggregated to `true` to return all events within the specified [`start`,`end`] timeframe. Otherwise if an event is aggregated to a parent event with a timestamp outside of the timeframe, it won't be available in the output. Aggregated events with `is_aggregate=true` in the response will still be returned unless exclude_aggregate is set to `true.` |  |
**exclude_aggregate** | Option<**bool**> | Set `exclude_aggregate` to `true` to only return unaggregated events where `is_aggregate=false` in the response. If the `exclude_aggregate` parameter is set to `true`, then the unaggregated parameter is ignored and will be `true` by default. |  |
**page** | Option<**i32**> | By default 1000 results are returned per request. Set page to the number of the page to return with `0` being the first page. The page parameter can only be used when either unaggregated or exclude_aggregate is set to `true.` |  |

### Return type

[**crate::models::EventListResponse**](EventListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

