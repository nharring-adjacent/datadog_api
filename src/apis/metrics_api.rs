/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `get_metric_metadata`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMetricMetadataError {
    Status403(crate::models::ApiErrorResponse),
    Status404(crate::models::ApiErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_active_metrics`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActiveMetricsError {
    Status400(crate::models::ApiErrorResponse),
    Status403(crate::models::ApiErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_metrics`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMetricsError {
    Status400(crate::models::ApiErrorResponse),
    Status403(crate::models::ApiErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `query_metrics`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryMetricsError {
    Status400(crate::models::ApiErrorResponse),
    Status403(crate::models::ApiErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `submit_metrics`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitMetricsError {
    Status400(crate::models::ApiErrorResponse),
    Status403(crate::models::ApiErrorResponse),
    Status408(crate::models::ApiErrorResponse),
    Status413(crate::models::ApiErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_metric_metadata`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMetricMetadataError {
    Status400(crate::models::ApiErrorResponse),
    Status403(crate::models::ApiErrorResponse),
    Status404(crate::models::ApiErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Get metadata about a specific metric.
pub async fn get_metric_metadata(configuration: &configuration::Configuration, metric_name: &str) -> Result<crate::models::MetricMetadata, Error<GetMetricMetadataError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/metrics/{metric_name}", configuration.base_path, metric_name=crate::apis::urlencode(metric_name));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("DD-API-KEY", local_var_value);
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("DD-APPLICATION-KEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetMetricMetadataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the list of actively reporting metrics from a given time until now.
pub async fn list_active_metrics(configuration: &configuration::Configuration, from: i64, host: Option<&str>, tag_filter: Option<&str>) -> Result<crate::models::MetricsListResponse, Error<ListActiveMetricsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/metrics", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("from", &from.to_string())]);
    if let Some(ref local_var_str) = host {
        local_var_req_builder = local_var_req_builder.query(&[("host", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tag_filter {
        local_var_req_builder = local_var_req_builder.query(&[("tag_filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("DD-API-KEY", local_var_value);
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("DD-APPLICATION-KEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListActiveMetricsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search for metrics from the last 24 hours in Datadog.
pub async fn list_metrics(configuration: &configuration::Configuration, q: &str) -> Result<crate::models::MetricSearchResponse, Error<ListMetricsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/search", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("q", &q.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("DD-API-KEY", local_var_value);
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("DD-APPLICATION-KEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListMetricsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Query timeseries points.
pub async fn query_metrics(configuration: &configuration::Configuration, from: i64, to: i64, query: &str) -> Result<crate::models::MetricsQueryResponse, Error<QueryMetricsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/query", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("from", &from.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("to", &to.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("query", &query.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("DD-API-KEY", local_var_value);
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("DD-APPLICATION-KEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QueryMetricsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The metrics end-point allows you to post time-series data that can be graphed on Datadog???s dashboards. The maximum payload size is 3.2 megabytes (3200000). Compressed payloads must have a decompressed size of up to 62 megabytes (62914560).  If you???re submitting metrics directly to the Datadog API without using DogStatsD, expect  - 64 bits for the timestamp - 32 bits for the value - 20 bytes for the metric names - 50 bytes for the timeseries - The full payload is approximately ~ 100 bytes. However, with the DogStatsD API, compression is applied, which reduces the payload size.
pub async fn submit_metrics(configuration: &configuration::Configuration, body: crate::models::MetricsPayload) -> Result<crate::models::IntakePayloadAccepted, Error<SubmitMetricsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/series", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("DD-API-KEY", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SubmitMetricsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Edit metadata of a specific metric. Find out more about [supported types](https://docs.datadoghq.com/developers/metrics).
pub async fn update_metric_metadata(configuration: &configuration::Configuration, metric_name: &str, body: crate::models::MetricMetadata) -> Result<crate::models::MetricMetadata, Error<UpdateMetricMetadataError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/metrics/{metric_name}", configuration.base_path, metric_name=crate::apis::urlencode(metric_name));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("DD-API-KEY", local_var_value);
    };
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("DD-APPLICATION-KEY", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateMetricMetadataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

