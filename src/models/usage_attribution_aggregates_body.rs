/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageAttributionAggregatesBody : The object containing the aggregates.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAttributionAggregatesBody {
    /// The aggregate type.
    #[serde(rename = "agg_type", skip_serializing_if = "Option::is_none")]
    pub agg_type: Option<String>,
    /// The field.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The value for a given field.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl UsageAttributionAggregatesBody {
    /// The object containing the aggregates.
    pub fn new() -> UsageAttributionAggregatesBody {
        UsageAttributionAggregatesBody {
            agg_type: None,
            field: None,
            value: None,
        }
    }
}


