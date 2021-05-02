/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsCoreWebVitals : Core Web Vitals attached to a browser test step.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCoreWebVitals {
    /// Cumulative Layout Shift.
    #[serde(rename = "cls", skip_serializing_if = "Option::is_none")]
    pub cls: Option<i64>,
    /// Largest Contentful Paint in milliseconds.
    #[serde(rename = "lcp", skip_serializing_if = "Option::is_none")]
    pub lcp: Option<i64>,
    /// URL attached to the metrics.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SyntheticsCoreWebVitals {
    /// Core Web Vitals attached to a browser test step.
    pub fn new() -> SyntheticsCoreWebVitals {
        SyntheticsCoreWebVitals {
            cls: None,
            lcp: None,
            url: None,
        }
    }
}


