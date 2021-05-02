# \KeyManagementApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](KeyManagementApi.md#create_api_key) | **post** /api/v1/api_key | Create an API key
[**create_application_key**](KeyManagementApi.md#create_application_key) | **post** /api/v1/application_key | Create an application key
[**delete_api_key**](KeyManagementApi.md#delete_api_key) | **delete** /api/v1/api_key/{key} | Delete an API key
[**delete_application_key**](KeyManagementApi.md#delete_application_key) | **delete** /api/v1/application_key/{key} | Delete an application key
[**get_api_key**](KeyManagementApi.md#get_api_key) | **get** /api/v1/api_key/{key} | Get API key
[**get_application_key**](KeyManagementApi.md#get_application_key) | **get** /api/v1/application_key/{key} | Get an application key
[**list_api_keys**](KeyManagementApi.md#list_api_keys) | **get** /api/v1/api_key | Get all API keys
[**list_application_keys**](KeyManagementApi.md#list_application_keys) | **get** /api/v1/application_key | Get all application keys
[**update_api_key**](KeyManagementApi.md#update_api_key) | **put** /api/v1/api_key/{key} | Edit an API key
[**update_application_key**](KeyManagementApi.md#update_application_key) | **put** /api/v1/application_key/{key} | Edit an application key



## create_api_key

> crate::models::ApiKeyResponse create_api_key(body)
Create an API key

Creates an API key with a given name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiKey**](ApiKey.md) |  | [required] |

### Return type

[**crate::models::ApiKeyResponse**](ApiKeyResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application_key

> crate::models::ApplicationKeyResponse create_application_key(body)
Create an application key

Create an application key with a given name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApplicationKey**](ApplicationKey.md) |  | [required] |

### Return type

[**crate::models::ApplicationKeyResponse**](ApplicationKeyResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> crate::models::ApiKeyResponse delete_api_key(key)
Delete an API key

Delete a given API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The specific API key you are working with. | [required] |

### Return type

[**crate::models::ApiKeyResponse**](ApiKeyResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_key

> crate::models::ApplicationKeyResponse delete_application_key(key)
Delete an application key

Delete a given application key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The specific APP key you are working with. | [required] |

### Return type

[**crate::models::ApplicationKeyResponse**](ApplicationKeyResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_key

> crate::models::ApiKeyResponse get_api_key(key)
Get API key

Get a given API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The specific API key you are working with. | [required] |

### Return type

[**crate::models::ApiKeyResponse**](ApiKeyResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_key

> crate::models::ApplicationKeyResponse get_application_key(key)
Get an application key

Get a given application key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The specific APP key you are working with. | [required] |

### Return type

[**crate::models::ApplicationKeyResponse**](ApplicationKeyResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_api_keys

> crate::models::ApiKeyListResponse list_api_keys()
Get all API keys

Get all API keys available for your account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiKeyListResponse**](ApiKeyListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_keys

> crate::models::ApplicationKeyListResponse list_application_keys()
Get all application keys

Get all application keys available for your Datadog account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApplicationKeyListResponse**](ApplicationKeyListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_key

> crate::models::ApiKeyResponse update_api_key(key, body)
Edit an API key

Edit an API key name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The specific API key you are working with. | [required] |
**body** | [**ApiKey**](ApiKey.md) |  | [required] |

### Return type

[**crate::models::ApiKeyResponse**](ApiKeyResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_key

> crate::models::ApplicationKeyResponse update_application_key(key, body)
Edit an application key

Edit an application key name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The specific APP key you are working with. | [required] |
**body** | [**ApplicationKey**](ApplicationKey.md) |  | [required] |

### Return type

[**crate::models::ApplicationKeyResponse**](ApplicationKeyResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

