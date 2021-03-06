/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsGlobalVariableParseTestOptionsType : Property of the Synthetics Test Response to use for a Synthetics global variable.

/// Property of the Synthetics Test Response to use for a Synthetics global variable.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsGlobalVariableParseTestOptionsType {
    #[serde(rename = "http_body")]
    HTTP_BODY,
    #[serde(rename = "http_header")]
    HTTP_HEADER,

}

impl ToString for SyntheticsGlobalVariableParseTestOptionsType {
    fn to_string(&self) -> String {
        match self {
            Self::HTTP_BODY => String::from("http_body"),
            Self::HTTP_HEADER => String::from("http_header"),
        }
    }
}




