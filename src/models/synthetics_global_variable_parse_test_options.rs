/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsGlobalVariableParseTestOptions : Parser options to use for retrieving a Synthetics global variable from a Synthetics Test. Used in conjunction with `parse_test_public_id`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGlobalVariableParseTestOptions {
    /// When type is `http_header`, name of the header to use to extract the value.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "parser")]
    pub parser: Box<crate::models::SyntheticsVariableParser>,
    #[serde(rename = "type")]
    pub _type: crate::models::SyntheticsGlobalVariableParseTestOptionsType,
}

impl SyntheticsGlobalVariableParseTestOptions {
    /// Parser options to use for retrieving a Synthetics global variable from a Synthetics Test. Used in conjunction with `parse_test_public_id`.
    pub fn new(parser: crate::models::SyntheticsVariableParser, _type: crate::models::SyntheticsGlobalVariableParseTestOptionsType) -> SyntheticsGlobalVariableParseTestOptions {
        SyntheticsGlobalVariableParseTestOptions {
            field: None,
            parser: Box::new(parser),
            _type,
        }
    }
}


