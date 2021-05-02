/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsServiceRemapperType : Type of logs service remapper.

/// Type of logs service remapper.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsServiceRemapperType {
    #[serde(rename = "service-remapper")]
    SERVICE_REMAPPER,

}

impl ToString for LogsServiceRemapperType {
    fn to_string(&self) -> String {
        match self {
            Self::SERVICE_REMAPPER => String::from("service-remapper"),
        }
    }
}




