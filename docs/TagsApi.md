# \TagsApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_host_tags**](TagsApi.md#create_host_tags) | **post** /api/v1/tags/hosts/{host_name} | Add tags to a host
[**delete_host_tags**](TagsApi.md#delete_host_tags) | **delete** /api/v1/tags/hosts/{host_name} | Remove host tags
[**get_host_tags**](TagsApi.md#get_host_tags) | **get** /api/v1/tags/hosts/{host_name} | Get host tags
[**list_host_tags**](TagsApi.md#list_host_tags) | **get** /api/v1/tags/hosts | Get Tags
[**update_host_tags**](TagsApi.md#update_host_tags) | **put** /api/v1/tags/hosts/{host_name} | Update host tags



## create_host_tags

> crate::models::HostTags create_host_tags(host_name, body, source)
Add tags to a host

This endpoint allows you to add new tags to a host, optionally specifying where these tags come from.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_name** | **String** | This endpoint allows you to add new tags to a host, optionally specifying where the tags came from. | [required] |
**body** | [**HostTags**](HostTags.md) | Update host tags request body. | [required] |
**source** | Option<**String**> | The source of the tags. [Complete list of source attribute values](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value). |  |

### Return type

[**crate::models::HostTags**](HostTags.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_host_tags

> delete_host_tags(host_name, source)
Remove host tags

This endpoint allows you to remove all user-assigned tags for a single host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_name** | **String** | This endpoint allows you to remove all user-assigned tags for a single host. | [required] |
**source** | Option<**String**> | The source of the tags (e.g. chef, puppet). [Complete list of source attribute values](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value). |  |

### Return type

 (empty response body)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_host_tags

> crate::models::HostTags get_host_tags(host_name, source)
Get host tags

Return the list of tags that apply to a given host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_name** | **String** | When specified, filters list of tags to those tags with the specified source. | [required] |
**source** | Option<**String**> | Source to filter. |  |

### Return type

[**crate::models::HostTags**](HostTags.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_host_tags

> crate::models::TagToHosts list_host_tags(source)
Get Tags

Return a mapping of tags to hosts for your whole infrastructure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source** | Option<**String**> | When specified, filters host list to those tags with the specified source. |  |

### Return type

[**crate::models::TagToHosts**](TagToHosts.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_host_tags

> crate::models::HostTags update_host_tags(host_name, body, source)
Update host tags

This endpoint allows you to update/replace all tags in an integration source with those supplied in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_name** | **String** | This endpoint allows you to update/replace all in an integration source with those supplied in the request. | [required] |
**body** | [**HostTags**](HostTags.md) | Add tags to host | [required] |
**source** | Option<**String**> | The source of the tags (e.g. chef, puppet). [Complete list of source attribute values](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value) |  |

### Return type

[**crate::models::HostTags**](HostTags.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

