/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsStatusRemapperType : Type of logs status remapper.

/// Type of logs status remapper.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsStatusRemapperType {
    #[serde(rename = "status-remapper")]
    STATUS_REMAPPER,

}

impl ToString for LogsStatusRemapperType {
    fn to_string(&self) -> String {
        match self {
            Self::STATUS_REMAPPER => String::from("status-remapper"),
        }
    }
}




