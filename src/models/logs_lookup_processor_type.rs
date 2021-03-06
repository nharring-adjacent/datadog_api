/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsLookupProcessorType : Type of logs lookup processor.

/// Type of logs lookup processor.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsLookupProcessorType {
    #[serde(rename = "lookup-processor")]
    LOOKUP_PROCESSOR,

}

impl ToString for LogsLookupProcessorType {
    fn to_string(&self) -> String {
        match self {
            Self::LOOKUP_PROCESSOR => String::from("lookup-processor"),
        }
    }
}




