/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloHistoryMetricsSeries : A representation of `metric` based SLO time series for the provided queries. This is the same response type from `batch_query` endpoint.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloHistoryMetricsSeries {
    /// Count of submitted metrics.
    #[serde(rename = "count")]
    pub count: i64,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::SloHistoryMetricsSeriesMetadata>,
    /// Total sum of the query.
    #[serde(rename = "sum")]
    pub sum: f64,
    /// The query values for each metric.
    #[serde(rename = "values")]
    pub values: Vec<f64>,
}

impl SloHistoryMetricsSeries {
    /// A representation of `metric` based SLO time series for the provided queries. This is the same response type from `batch_query` endpoint.
    pub fn new(count: i64, metadata: crate::models::SloHistoryMetricsSeriesMetadata, sum: f64, values: Vec<f64>) -> SloHistoryMetricsSeries {
        SloHistoryMetricsSeries {
            count,
            metadata: Box::new(metadata),
            sum,
            values,
        }
    }
}


