/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// PagerDutyService : The PagerDuty service that is available for integration with Datadog.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagerDutyService {
    /// Your service key in PagerDuty.
    #[serde(rename = "service_key")]
    pub service_key: String,
    /// Your service name associated with a service key in PagerDuty.
    #[serde(rename = "service_name")]
    pub service_name: String,
}

impl PagerDutyService {
    /// The PagerDuty service that is available for integration with Datadog.
    pub fn new(service_key: String, service_name: String) -> PagerDutyService {
        PagerDutyService {
            service_key,
            service_name,
        }
    }
}


