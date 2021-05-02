/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// FormulaAndFunctionEventQueryGroupBySort : Options for sorting group by results.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionEventQueryGroupBySort {
    #[serde(rename = "aggregation")]
    pub aggregation: crate::models::FormulaAndFunctionEventAggregation,
    /// Metric used for sorting group by results.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<crate::models::QuerySortOrder>,
}

impl FormulaAndFunctionEventQueryGroupBySort {
    /// Options for sorting group by results.
    pub fn new(aggregation: crate::models::FormulaAndFunctionEventAggregation) -> FormulaAndFunctionEventQueryGroupBySort {
        FormulaAndFunctionEventQueryGroupBySort {
            aggregation,
            metric: None,
            order: None,
        }
    }
}

