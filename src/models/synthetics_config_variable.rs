/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsConfigVariable : Object defining a variable that can be used in your test configuration.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsConfigVariable {
    /// Example for the variable.
    #[serde(rename = "example")]
    pub example: String,
    /// Name of the variable.
    #[serde(rename = "name")]
    pub name: String,
    /// Pattern of the variable.
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "type")]
    pub _type: crate::models::SyntheticsConfigVariableType,
}

impl SyntheticsConfigVariable {
    /// Object defining a variable that can be used in your test configuration.
    pub fn new(example: String, name: String, _type: crate::models::SyntheticsConfigVariableType) -> SyntheticsConfigVariable {
        SyntheticsConfigVariable {
            example,
            name,
            pattern: None,
            _type,
        }
    }
}


