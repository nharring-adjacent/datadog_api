/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloCorrectionResponse : The response object of an SLO correction



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloCorrectionResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::SloCorrection>>,
}

impl SloCorrectionResponse {
    /// The response object of an SLO correction
    pub fn new() -> SloCorrectionResponse {
        SloCorrectionResponse {
            data: None,
        }
    }
}


