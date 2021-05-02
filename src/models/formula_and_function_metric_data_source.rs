/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// FormulaAndFunctionMetricDataSource : Data source for metrics queries.

/// Data source for metrics queries.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormulaAndFunctionMetricDataSource {
    #[serde(rename = "metrics")]
    METRICS,

}

impl ToString for FormulaAndFunctionMetricDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::METRICS => String::from("metrics"),
        }
    }
}




