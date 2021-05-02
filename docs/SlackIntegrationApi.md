# \SlackIntegrationApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_slack_integration_channel**](SlackIntegrationApi.md#create_slack_integration_channel) | **post** /api/v1/integration/slack/configuration/accounts/{account_name}/channels | Create a Slack integration channel
[**get_slack_integration_channel**](SlackIntegrationApi.md#get_slack_integration_channel) | **get** /api/v1/integration/slack/configuration/accounts/{account_name}/channels/{channel_name} | Get a Slack integration channel
[**get_slack_integration_channels**](SlackIntegrationApi.md#get_slack_integration_channels) | **get** /api/v1/integration/slack/configuration/accounts/{account_name}/channels | Get all channels in a Slack integration
[**remove_slack_integration_channel**](SlackIntegrationApi.md#remove_slack_integration_channel) | **delete** /api/v1/integration/slack/configuration/accounts/{account_name}/channels/{channel_name} | Remove a Slack integration channel
[**update_slack_integration_channel**](SlackIntegrationApi.md#update_slack_integration_channel) | **patch** /api/v1/integration/slack/configuration/accounts/{account_name}/channels/{channel_name} | Update a Slack integration channel



## create_slack_integration_channel

> crate::models::SlackIntegrationChannel create_slack_integration_channel(account_name, body)
Create a Slack integration channel

Add a channel to your Datadog-Slack integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** | Your Slack account name. | [required] |
**body** | [**SlackIntegrationChannel**](SlackIntegrationChannel.md) | Payload describing Slack channel to be created | [required] |

### Return type

[**crate::models::SlackIntegrationChannel**](SlackIntegrationChannel.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_slack_integration_channel

> crate::models::SlackIntegrationChannel get_slack_integration_channel(account_name, channel_name)
Get a Slack integration channel

Get a channel configured for your Datadog-Slack integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** | Your Slack account name. | [required] |
**channel_name** | **String** | The name of the Slack channel being operated on. | [required] |

### Return type

[**crate::models::SlackIntegrationChannel**](SlackIntegrationChannel.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_slack_integration_channels

> Vec<crate::models::SlackIntegrationChannel> get_slack_integration_channels(account_name)
Get all channels in a Slack integration

Get a list of all channels configured for your Datadog-Slack integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** | Your Slack account name. | [required] |

### Return type

[**Vec<crate::models::SlackIntegrationChannel>**](SlackIntegrationChannel.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_slack_integration_channel

> remove_slack_integration_channel(account_name, channel_name)
Remove a Slack integration channel

Remove a channel from your Datadog-Slack integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** | Your Slack account name. | [required] |
**channel_name** | **String** | The name of the Slack channel being operated on. | [required] |

### Return type

 (empty response body)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_slack_integration_channel

> crate::models::SlackIntegrationChannel update_slack_integration_channel(account_name, channel_name, body)
Update a Slack integration channel

Update a channel used in your Datadog-Slack integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** | Your Slack account name. | [required] |
**channel_name** | **String** | The name of the Slack channel being operated on. | [required] |
**body** | [**SlackIntegrationChannel**](SlackIntegrationChannel.md) | Payload describing fields and values to be updated. | [required] |

### Return type

[**crate::models::SlackIntegrationChannel**](SlackIntegrationChannel.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

