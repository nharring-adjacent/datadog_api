/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsVariableParser : Details of the parser to use for the global variable.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsVariableParser {
    #[serde(rename = "type")]
    pub _type: crate::models::SyntheticsGlobalVariableParserType,
    /// Regex or JSON path used for the parser. Not used with type `raw`.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl SyntheticsVariableParser {
    /// Details of the parser to use for the global variable.
    pub fn new(_type: crate::models::SyntheticsGlobalVariableParserType) -> SyntheticsVariableParser {
        SyntheticsVariableParser {
            _type,
            value: None,
        }
    }
}

