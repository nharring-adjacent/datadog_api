/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsApiStep : The steps used in a Synthetics multistep API test.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsApiStep {
    /// Determines whether or not to continue with test if this step fails.
    #[serde(rename = "allowFailure", skip_serializing_if = "Option::is_none")]
    pub allow_failure: Option<bool>,
    /// Array of assertions used for the test.
    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Option<Vec<crate::models::SyntheticsAssertion>>,
    /// Array of values to parse and save as variables from the response.
    #[serde(rename = "extractedValues", skip_serializing_if = "Option::is_none")]
    pub extracted_values: Option<Vec<crate::models::SyntheticsParsingOptions>>,
    /// Determines whether or not to consider the entire test as failed if this step fails. Can be used only if `allowFailure` is `true`.
    #[serde(rename = "isCritical", skip_serializing_if = "Option::is_none")]
    pub is_critical: Option<bool>,
    /// The name of the step.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: Option<Box<crate::models::SyntheticsTestRequest>>,
    #[serde(rename = "subtype", skip_serializing_if = "Option::is_none")]
    pub subtype: Option<crate::models::SyntheticsApiStepSubtype>,
}

impl SyntheticsApiStep {
    /// The steps used in a Synthetics multistep API test.
    pub fn new() -> SyntheticsApiStep {
        SyntheticsApiStep {
            allow_failure: None,
            assertions: None,
            extracted_values: None,
            is_critical: None,
            name: None,
            request: None,
            subtype: None,
        }
    }
}


