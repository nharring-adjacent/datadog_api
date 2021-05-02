/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// OrganizationSubscription : Subscription definition.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSubscription {
    /// The subscription type. Types available are `trial`, `free`, and `pro`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl OrganizationSubscription {
    /// Subscription definition.
    pub fn new() -> OrganizationSubscription {
        OrganizationSubscription {
            _type: None,
        }
    }
}

