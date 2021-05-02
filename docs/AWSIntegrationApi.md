# \AWSIntegrationApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_aws_account**](AWSIntegrationApi.md#create_aws_account) | **post** /api/v1/integration/aws | Create an AWS integration
[**create_aws_tag_filter**](AWSIntegrationApi.md#create_aws_tag_filter) | **post** /api/v1/integration/aws/filtering | Set an AWS tag filter
[**create_new_aws_external_id**](AWSIntegrationApi.md#create_new_aws_external_id) | **put** /api/v1/integration/aws/generate_new_external_id | Generate a new external ID
[**delete_aws_account**](AWSIntegrationApi.md#delete_aws_account) | **delete** /api/v1/integration/aws | Delete an AWS integration
[**delete_aws_tag_filter**](AWSIntegrationApi.md#delete_aws_tag_filter) | **delete** /api/v1/integration/aws/filtering | Delete a tag filtering entry
[**list_available_aws_namespaces**](AWSIntegrationApi.md#list_available_aws_namespaces) | **get** /api/v1/integration/aws/available_namespace_rules | List namespace rules
[**list_aws_accounts**](AWSIntegrationApi.md#list_aws_accounts) | **get** /api/v1/integration/aws | List all AWS integrations
[**list_aws_tag_filters**](AWSIntegrationApi.md#list_aws_tag_filters) | **get** /api/v1/integration/aws/filtering | Get all AWS tag filters
[**update_aws_account**](AWSIntegrationApi.md#update_aws_account) | **put** /api/v1/integration/aws | Update an AWS integration



## create_aws_account

> crate::models::AwsAccountCreateResponse create_aws_account(body)
Create an AWS integration

Create a Datadog-Amazon Web Services integration. Using the `POST` method updates your integration configuration by adding your new configuration to the existing one in your Datadog organization. A unique AWS Account ID for role based authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AwsAccount**](AwsAccount.md) | AWS Request Object | [required] |

### Return type

[**crate::models::AwsAccountCreateResponse**](AWSAccountCreateResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_aws_tag_filter

> serde_json::Value create_aws_tag_filter(body)
Set an AWS tag filter

Set an AWS tag filter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AwsTagFilterCreateRequest**](AwsTagFilterCreateRequest.md) | Set an AWS tag filter using an `aws_account_identifier`, `namespace`, and filtering string. Namespace options are `application_elb`, `elb`, `lambda`, `network_elb`, `rds`, `sqs`, and `custom`. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_new_aws_external_id

> crate::models::AwsAccountCreateResponse create_new_aws_external_id(body)
Generate a new external ID

Generate a new AWS external ID for a given AWS account ID and role name pair.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AwsAccount**](AwsAccount.md) | Your Datadog role delegation name. For more information about your AWS account Role name, see the [Datadog AWS integration configuration info](https://github.com/DataDog/documentation/blob/master/integrations/amazon_web_services/#installation). | [required] |

### Return type

[**crate::models::AwsAccountCreateResponse**](AWSAccountCreateResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_aws_account

> serde_json::Value delete_aws_account(body)
Delete an AWS integration

Delete a Datadog-AWS integration matching the specified `account_id` and `role_name parameters`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AwsAccount**](AwsAccount.md) | AWS request object | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_aws_tag_filter

> serde_json::Value delete_aws_tag_filter(body)
Delete a tag filtering entry

Delete a tag filtering entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AwsTagFilterDeleteRequest**](AwsTagFilterDeleteRequest.md) | Delete a tag filtering entry for a given AWS account and `dd-aws` namespace. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_aws_namespaces

> Vec<String> list_available_aws_namespaces()
List namespace rules

List all namespace rules for a given Datadog-AWS integration. This endpoint takes no arguments.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_accounts

> crate::models::AwsAccountListResponse list_aws_accounts(account_id, role_name, access_key_id)
List all AWS integrations

List all Datadog-AWS integrations available in your Datadog organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**String**> | Only return AWS accounts that matches this `account_id`. |  |
**role_name** | Option<**String**> | Only return AWS accounts that matches this role_name. |  |
**access_key_id** | Option<**String**> | Only return AWS accounts that matches this `access_key_id`. |  |

### Return type

[**crate::models::AwsAccountListResponse**](AWSAccountListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_tag_filters

> crate::models::AwsTagFilterListResponse list_aws_tag_filters(account_id)
Get all AWS tag filters

Get all AWS tag filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Only return AWS filters that matches this `account_id`. | [required] |

### Return type

[**crate::models::AwsTagFilterListResponse**](AWSTagFilterListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_aws_account

> serde_json::Value update_aws_account(body, account_id, role_name, access_key_id)
Update an AWS integration

Update a Datadog-Amazon Web Services integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AwsAccount**](AwsAccount.md) | AWS request object | [required] |
**account_id** | Option<**String**> | Only return AWS accounts that matches this `account_id`. |  |
**role_name** | Option<**String**> | Only return AWS accounts that match this `role_name`. Required if `account_id` is specified. |  |
**access_key_id** | Option<**String**> | Only return AWS accounts that matches this `access_key_id`. Required if none of the other two options are specified. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

