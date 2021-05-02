/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsDevice : Object describing the device used to perform the Synthetic test.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsDevice {
    /// Screen height of the device.
    #[serde(rename = "height")]
    pub height: i64,
    #[serde(rename = "id")]
    pub id: crate::models::SyntheticsDeviceId,
    /// Whether or not the device is a mobile.
    #[serde(rename = "isMobile", skip_serializing_if = "Option::is_none")]
    pub is_mobile: Option<bool>,
    /// The device name.
    #[serde(rename = "name")]
    pub name: String,
    /// Screen width of the device.
    #[serde(rename = "width")]
    pub width: i64,
}

impl SyntheticsDevice {
    /// Object describing the device used to perform the Synthetic test.
    pub fn new(height: i64, id: crate::models::SyntheticsDeviceId, name: String, width: i64) -> SyntheticsDevice {
        SyntheticsDevice {
            height,
            id,
            is_mobile: None,
            name,
            width,
        }
    }
}


