/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageSnmpHour : The number of SNMP devices for each hour for a given organization.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSnmpHour {
    /// The hour for the usage.
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: Option<String>,
    /// Contains the number of SNMP devices.
    #[serde(rename = "snmp_devices", skip_serializing_if = "Option::is_none")]
    pub snmp_devices: Option<i64>,
}

impl UsageSnmpHour {
    /// The number of SNMP devices for each hour for a given organization.
    pub fn new() -> UsageSnmpHour {
        UsageSnmpHour {
            hour: None,
            snmp_devices: None,
        }
    }
}


