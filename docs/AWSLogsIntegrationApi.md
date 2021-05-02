# \AWSLogsIntegrationApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_aws_logs_lambda_async**](AWSLogsIntegrationApi.md#check_aws_logs_lambda_async) | **post** /api/v1/integration/aws/logs/check_async | Check that an AWS Lambda Function exists
[**check_aws_logs_services_async**](AWSLogsIntegrationApi.md#check_aws_logs_services_async) | **post** /api/v1/integration/aws/logs/services_async | Check permissions for log services
[**create_aws_lambda_arn**](AWSLogsIntegrationApi.md#create_aws_lambda_arn) | **post** /api/v1/integration/aws/logs | Add AWS Log Lambda ARN
[**delete_aws_lambda_arn**](AWSLogsIntegrationApi.md#delete_aws_lambda_arn) | **delete** /api/v1/integration/aws/logs | Delete an AWS Logs integration
[**enable_aws_log_services**](AWSLogsIntegrationApi.md#enable_aws_log_services) | **post** /api/v1/integration/aws/logs/services | Enable an AWS Logs integration
[**list_aws_logs_integrations**](AWSLogsIntegrationApi.md#list_aws_logs_integrations) | **get** /api/v1/integration/aws/logs | List all AWS Logs integrations
[**list_aws_logs_services**](AWSLogsIntegrationApi.md#list_aws_logs_services) | **get** /api/v1/integration/aws/logs/services | Get list of AWS log ready services



## check_aws_logs_lambda_async

> crate::models::AwsLogsAsyncResponse check_aws_logs_lambda_async(body)
Check that an AWS Lambda Function exists

Test if permissions are present to add a log-forwarding triggers for the given services and AWS account. The input is the same as for Enable an AWS service log collection. Subsequent requests will always repeat the above, so this endpoint can be polled intermittently instead of blocking.  - Returns a status of 'created' when it's checking if the Lambda exists in the account. - Returns a status of 'waiting' while checking. - Returns a status of 'checked and ok' if the Lambda exists. - Returns a status of 'error' if the Lambda does not exist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AwsAccountAndLambdaRequest**](AwsAccountAndLambdaRequest.md) | Check AWS Log Lambda Async request body. | [required] |

### Return type

[**crate::models::AwsLogsAsyncResponse**](AWSLogsAsyncResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_aws_logs_services_async

> crate::models::AwsLogsAsyncResponse check_aws_logs_services_async(body)
Check permissions for log services

Test if permissions are present to add log-forwarding triggers for the given services and AWS account. Input is the same as for `EnableAWSLogServices`. Done async, so can be repeatedly polled in a non-blocking fashion until the async request completes.  - Returns a status of `created` when it's checking if the permissions exists   in the AWS account. - Returns a status of `waiting` while checking. - Returns a status of `checked and ok` if the Lambda exists. - Returns a status of `error` if the Lambda does not exist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AwsLogsServicesRequest**](AwsLogsServicesRequest.md) | Check AWS Logs Async Services request body. | [required] |

### Return type

[**crate::models::AwsLogsAsyncResponse**](AWSLogsAsyncResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_aws_lambda_arn

> serde_json::Value create_aws_lambda_arn(body)
Add AWS Log Lambda ARN

Attach the Lambda ARN of the Lambda created for the Datadog-AWS log collection to your AWS account ID to enable log collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AwsAccountAndLambdaRequest**](AwsAccountAndLambdaRequest.md) | AWS Log Lambda Async request body. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_aws_lambda_arn

> serde_json::Value delete_aws_lambda_arn(body)
Delete an AWS Logs integration

Delete a Datadog-AWS logs configuration by removing the specific Lambda ARN associated with a given AWS account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AwsAccountAndLambdaRequest**](AwsAccountAndLambdaRequest.md) | Delete AWS Lambda ARN request body. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_aws_log_services

> serde_json::Value enable_aws_log_services(body)
Enable an AWS Logs integration

Enable automatic log collection for a list of services. This should be run after running `CreateAWSLambdaARN` to save the configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AwsLogsServicesRequest**](AwsLogsServicesRequest.md) | Enable AWS Log Services request body. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_logs_integrations

> Vec<crate::models::AwsLogsListResponse> list_aws_logs_integrations()
List all AWS Logs integrations

List all Datadog-AWS Logs integrations configured in your Datadog account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AwsLogsListResponse>**](AWSLogsListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_logs_services

> Vec<crate::models::AwsLogsListServicesResponse> list_aws_logs_services()
Get list of AWS log ready services

Get the list of current AWS services that Datadog offers automatic log collection. Use returned service IDs with the services parameter for the Enable an AWS service log collection API endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AwsLogsListServicesResponse>**](AWSLogsListServicesResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

