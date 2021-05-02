/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// FormulaAndFunctionEventQueryDefinitionCompute : Compute options.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionEventQueryDefinitionCompute {
    #[serde(rename = "aggregation")]
    pub aggregation: crate::models::FormulaAndFunctionEventAggregation,
    /// A time interval in milliseconds.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// Measurable attribute to compute.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
}

impl FormulaAndFunctionEventQueryDefinitionCompute {
    /// Compute options.
    pub fn new(aggregation: crate::models::FormulaAndFunctionEventAggregation) -> FormulaAndFunctionEventQueryDefinitionCompute {
        FormulaAndFunctionEventQueryDefinitionCompute {
            aggregation,
            interval: None,
            metric: None,
        }
    }
}


