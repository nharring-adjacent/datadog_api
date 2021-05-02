/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsCiTestBody : Object describing the synthetics tests to trigger.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCiTestBody {
    /// Individual synthetics test.
    #[serde(rename = "tests", skip_serializing_if = "Option::is_none")]
    pub tests: Option<Vec<crate::models::SyntheticsCiTest>>,
}

impl SyntheticsCiTestBody {
    /// Object describing the synthetics tests to trigger.
    pub fn new() -> SyntheticsCiTestBody {
        SyntheticsCiTestBody {
            tests: None,
        }
    }
}


