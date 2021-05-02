/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsAssertionJsonPathOperator : Assertion operator to apply.

/// Assertion operator to apply.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsAssertionJsonPathOperator {
    #[serde(rename = "validatesJSONPath")]
    VALIDATES_JSON_PATH,

}

impl ToString for SyntheticsAssertionJsonPathOperator {
    fn to_string(&self) -> String {
        match self {
            Self::VALIDATES_JSON_PATH => String::from("validatesJSONPath"),
        }
    }
}



