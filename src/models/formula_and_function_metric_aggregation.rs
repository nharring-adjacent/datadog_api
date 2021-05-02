/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// FormulaAndFunctionMetricAggregation : The aggregation methods available for metrics queries.

/// The aggregation methods available for metrics queries.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormulaAndFunctionMetricAggregation {
    #[serde(rename = "avg")]
    AVG,
    #[serde(rename = "min")]
    MIN,
    #[serde(rename = "max")]
    MAX,
    #[serde(rename = "sum")]
    SUM,
    #[serde(rename = "last")]
    LAST,
    #[serde(rename = "area")]
    AREA,
    #[serde(rename = "l2norm")]
    L2NORM,

}

impl ToString for FormulaAndFunctionMetricAggregation {
    fn to_string(&self) -> String {
        match self {
            Self::AVG => String::from("avg"),
            Self::MIN => String::from("min"),
            Self::MAX => String::from("max"),
            Self::SUM => String::from("sum"),
            Self::LAST => String::from("last"),
            Self::AREA => String::from("area"),
            Self::L2NORM => String::from("l2norm"),
        }
    }
}




