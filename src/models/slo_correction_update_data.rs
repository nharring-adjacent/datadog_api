/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloCorrectionUpdateData : The data object associated with the SLO correction to be updated



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloCorrectionUpdateData {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::SloCorrectionUpdateRequestAttributes>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::SloCorrectionType>,
}

impl SloCorrectionUpdateData {
    /// The data object associated with the SLO correction to be updated
    pub fn new() -> SloCorrectionUpdateData {
        SloCorrectionUpdateData {
            attributes: None,
            _type: None,
        }
    }
}


