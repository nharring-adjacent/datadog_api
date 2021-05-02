/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsGrokParserRules : Set of rules for the grok parser.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsGrokParserRules {
    /// List of match rules for the grok parser, separated by a new line.
    #[serde(rename = "match_rules")]
    pub match_rules: String,
    /// List of support rules for the grok parser, separated by a new line.
    #[serde(rename = "support_rules", skip_serializing_if = "Option::is_none")]
    pub support_rules: Option<String>,
}

impl LogsGrokParserRules {
    /// Set of rules for the grok parser.
    pub fn new(match_rules: String) -> LogsGrokParserRules {
        LogsGrokParserRules {
            match_rules,
            support_rules: None,
        }
    }
}


