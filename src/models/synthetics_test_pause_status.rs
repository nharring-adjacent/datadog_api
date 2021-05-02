/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsTestPauseStatus : Define whether you want to start (`live`) or pause (`paused`) a Synthetic test.

/// Define whether you want to start (`live`) or pause (`paused`) a Synthetic test.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsTestPauseStatus {
    #[serde(rename = "live")]
    LIVE,
    #[serde(rename = "paused")]
    PAUSED,

}

impl ToString for SyntheticsTestPauseStatus {
    fn to_string(&self) -> String {
        match self {
            Self::LIVE => String::from("live"),
            Self::PAUSED => String::from("paused"),
        }
    }
}




