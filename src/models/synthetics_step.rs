/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsStep : The steps used in a Synthetics browser test.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsStep {
    /// A boolean set to allow this step to fail.
    #[serde(rename = "allowFailure", skip_serializing_if = "Option::is_none")]
    pub allow_failure: Option<bool>,
    /// The name of the step.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The parameters of the step.
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    /// The time before declaring a step failed.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::SyntheticsStepType>,
}

impl SyntheticsStep {
    /// The steps used in a Synthetics browser test.
    pub fn new() -> SyntheticsStep {
        SyntheticsStep {
            allow_failure: None,
            name: None,
            params: None,
            timeout: None,
            _type: None,
        }
    }
}

