# \LogsPipelinesApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_logs_pipeline**](LogsPipelinesApi.md#create_logs_pipeline) | **post** /api/v1/logs/config/pipelines | Create a pipeline
[**delete_logs_pipeline**](LogsPipelinesApi.md#delete_logs_pipeline) | **delete** /api/v1/logs/config/pipelines/{pipeline_id} | Delete a pipeline
[**get_logs_pipeline**](LogsPipelinesApi.md#get_logs_pipeline) | **get** /api/v1/logs/config/pipelines/{pipeline_id} | Get a pipeline
[**get_logs_pipeline_order**](LogsPipelinesApi.md#get_logs_pipeline_order) | **get** /api/v1/logs/config/pipeline-order | Get pipeline order
[**list_logs_pipelines**](LogsPipelinesApi.md#list_logs_pipelines) | **get** /api/v1/logs/config/pipelines | Get all pipelines
[**update_logs_pipeline**](LogsPipelinesApi.md#update_logs_pipeline) | **put** /api/v1/logs/config/pipelines/{pipeline_id} | Update a pipeline
[**update_logs_pipeline_order**](LogsPipelinesApi.md#update_logs_pipeline_order) | **put** /api/v1/logs/config/pipeline-order | Update pipeline order



## create_logs_pipeline

> crate::models::LogsPipeline create_logs_pipeline(body)
Create a pipeline

Create a pipeline in your organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LogsPipeline**](LogsPipeline.md) | Definition of the new pipeline. | [required] |

### Return type

[**crate::models::LogsPipeline**](LogsPipeline.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_logs_pipeline

> delete_logs_pipeline(pipeline_id)
Delete a pipeline

Delete a given pipeline from your organization. This endpoint takes no JSON arguments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** | ID of the pipeline to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logs_pipeline

> crate::models::LogsPipeline get_logs_pipeline(pipeline_id)
Get a pipeline

Get a specific pipeline from your organization. This endpoint takes no JSON arguments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** | ID of the pipeline to get. | [required] |

### Return type

[**crate::models::LogsPipeline**](LogsPipeline.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logs_pipeline_order

> crate::models::LogsPipelinesOrder get_logs_pipeline_order()
Get pipeline order

Get the current order of your pipelines. This endpoint takes no JSON arguments.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LogsPipelinesOrder**](LogsPipelinesOrder.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_logs_pipelines

> Vec<crate::models::LogsPipeline> list_logs_pipelines()
Get all pipelines

Get all pipelines from your organization. This endpoint takes no JSON arguments.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LogsPipeline>**](LogsPipeline.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_logs_pipeline

> crate::models::LogsPipeline update_logs_pipeline(pipeline_id, body)
Update a pipeline

Update a given pipeline configuration to change it’s processors or their order.  **Note**: Using this method updates your pipeline configuration by **replacing** your current configuration with the new one sent to your Datadog organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** | ID of the pipeline to delete. | [required] |
**body** | [**LogsPipeline**](LogsPipeline.md) | New definition of the pipeline. | [required] |

### Return type

[**crate::models::LogsPipeline**](LogsPipeline.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_logs_pipeline_order

> crate::models::LogsPipelinesOrder update_logs_pipeline_order(body)
Update pipeline order

Update the order of your pipelines. Since logs are processed sequentially, reordering a pipeline may change the structure and content of the data processed by other pipelines and their processors.  **Note**: Using the `PUT` method updates your pipeline order by replacing your current order with the new one sent to your Datadog organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LogsPipelinesOrder**](LogsPipelinesOrder.md) | Object containing the new ordered list of pipeline IDs. | [required] |

### Return type

[**crate::models::LogsPipelinesOrder**](LogsPipelinesOrder.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

