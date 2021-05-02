/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsUrlParserType : Type of logs URL parser.

/// Type of logs URL parser.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsUrlParserType {
    #[serde(rename = "url-parser")]
    URL_PARSER,

}

impl ToString for LogsUrlParserType {
    fn to_string(&self) -> String {
        match self {
            Self::URL_PARSER => String::from("url-parser"),
        }
    }
}



