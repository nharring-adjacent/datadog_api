/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsUrlParser : This processor extracts query parameters and other important parameters from a URL.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsUrlParser {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Normalize the ending slashes or not.
    #[serde(rename = "normalize_ending_slashes", skip_serializing_if = "Option::is_none")]
    pub normalize_ending_slashes: Option<bool>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Name of the parent attribute that contains all the extracted details from the `sources`.
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "type")]
    pub _type: crate::models::LogsUrlParserType,
}

impl LogsUrlParser {
    /// This processor extracts query parameters and other important parameters from a URL.
    pub fn new(sources: Vec<String>, target: String, _type: crate::models::LogsUrlParserType) -> LogsUrlParser {
        LogsUrlParser {
            is_enabled: None,
            name: None,
            normalize_ending_slashes: None,
            sources,
            target,
            _type,
        }
    }
}

