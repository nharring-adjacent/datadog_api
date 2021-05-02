/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsCheckType : Type of assertion to apply in an API test.

/// Type of assertion to apply in an API test.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsCheckType {
    #[serde(rename = "equals")]
    EQUALS,
    #[serde(rename = "notEquals")]
    NOT_EQUALS,
    #[serde(rename = "contains")]
    CONTAINS,
    #[serde(rename = "notContains")]
    NOT_CONTAINS,
    #[serde(rename = "startsWith")]
    STARTS_WITH,
    #[serde(rename = "notStartsWith")]
    NOT_STARTS_WITH,
    #[serde(rename = "greater")]
    GREATER,
    #[serde(rename = "lower")]
    LOWER,
    #[serde(rename = "greaterEquals")]
    GREATER_EQUALS,
    #[serde(rename = "lowerEquals")]
    LOWER_EQUALS,
    #[serde(rename = "matchRegex")]
    MATCH_REGEX,
    #[serde(rename = "between")]
    BETWEEN,
    #[serde(rename = "isEmpty")]
    IS_EMPTY,
    #[serde(rename = "notIsEmpty")]
    NOT_IS_EMPTY,

}

impl ToString for SyntheticsCheckType {
    fn to_string(&self) -> String {
        match self {
            Self::EQUALS => String::from("equals"),
            Self::NOT_EQUALS => String::from("notEquals"),
            Self::CONTAINS => String::from("contains"),
            Self::NOT_CONTAINS => String::from("notContains"),
            Self::STARTS_WITH => String::from("startsWith"),
            Self::NOT_STARTS_WITH => String::from("notStartsWith"),
            Self::GREATER => String::from("greater"),
            Self::LOWER => String::from("lower"),
            Self::GREATER_EQUALS => String::from("greaterEquals"),
            Self::LOWER_EQUALS => String::from("lowerEquals"),
            Self::MATCH_REGEX => String::from("matchRegex"),
            Self::BETWEEN => String::from("between"),
            Self::IS_EMPTY => String::from("isEmpty"),
            Self::NOT_IS_EMPTY => String::from("notIsEmpty"),
        }
    }
}




