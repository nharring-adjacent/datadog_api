/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsLookupProcessor : Use the Lookup Processor to define a mapping between a log attribute and a human readable value saved in the processors mapping table. For example, you can use the Lookup Processor to map an internal service ID into a human readable service name. Alternatively, you could also use it to check if the MAC address that just attempted to connect to the production environment belongs to your list of stolen machines.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsLookupProcessor {
    /// Value to set the target attribute if the source value is not found in the list.
    #[serde(rename = "default_lookup", skip_serializing_if = "Option::is_none")]
    pub default_lookup: Option<String>,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// Mapping table of values for the source attribute and their associated target attribute values, formatted as `[\"source_key1,target_value1\", \"source_key2,target_value2\"]`
    #[serde(rename = "lookup_table")]
    pub lookup_table: Vec<String>,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Source attribute used to perform the lookup.
    #[serde(rename = "source")]
    pub source: String,
    /// Name of the attribute that contains the corresponding value in the mapping list or the `default_lookup` if not found in the mapping list.
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "type")]
    pub _type: crate::models::LogsLookupProcessorType,
}

impl LogsLookupProcessor {
    /// Use the Lookup Processor to define a mapping between a log attribute and a human readable value saved in the processors mapping table. For example, you can use the Lookup Processor to map an internal service ID into a human readable service name. Alternatively, you could also use it to check if the MAC address that just attempted to connect to the production environment belongs to your list of stolen machines.
    pub fn new(lookup_table: Vec<String>, source: String, target: String, _type: crate::models::LogsLookupProcessorType) -> LogsLookupProcessor {
        LogsLookupProcessor {
            default_lookup: None,
            is_enabled: None,
            lookup_table,
            name: None,
            source,
            target,
            _type,
        }
    }
}


