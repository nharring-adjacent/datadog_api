/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsTestProcessStatus : Status of a Synthetic test.

/// Status of a Synthetic test.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsTestProcessStatus {
    #[serde(rename = "not_scheduled")]
    NOT_SCHEDULED,
    #[serde(rename = "scheduled")]
    SCHEDULED,
    #[serde(rename = "started")]
    STARTED,
    #[serde(rename = "finished")]
    FINISHED,
    #[serde(rename = "finished_with_error")]
    FINISHED_WITH_ERROR,

}

impl ToString for SyntheticsTestProcessStatus {
    fn to_string(&self) -> String {
        match self {
            Self::NOT_SCHEDULED => String::from("not_scheduled"),
            Self::SCHEDULED => String::from("scheduled"),
            Self::STARTED => String::from("started"),
            Self::FINISHED => String::from("finished"),
            Self::FINISHED_WITH_ERROR => String::from("finished_with_error"),
        }
    }
}




