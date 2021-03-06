/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MonitorOverallStates : The different states your monitor can be in.

/// The different states your monitor can be in.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MonitorOverallStates {
    #[serde(rename = "Alert")]
    ALERT,
    #[serde(rename = "Ignored")]
    IGNORED,
    #[serde(rename = "No Data")]
    NO_DATA,
    #[serde(rename = "OK")]
    OK,
    #[serde(rename = "Skipped")]
    SKIPPED,
    #[serde(rename = "Unknown")]
    UNKNOWN,
    #[serde(rename = "Warn")]
    WARN,

}

impl ToString for MonitorOverallStates {
    fn to_string(&self) -> String {
        match self {
            Self::ALERT => String::from("Alert"),
            Self::IGNORED => String::from("Ignored"),
            Self::NO_DATA => String::from("No Data"),
            Self::OK => String::from("OK"),
            Self::SKIPPED => String::from("Skipped"),
            Self::UNKNOWN => String::from("Unknown"),
            Self::WARN => String::from("Warn"),
        }
    }
}




