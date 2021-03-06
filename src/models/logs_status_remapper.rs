/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsStatusRemapper : Use this Processor if you want to assign some attributes as the official status.  Each incoming status value is mapped as follows.    - Integers from 0 to 7 map to the Syslog severity standards   - Strings beginning with `emerg` or f (case-insensitive) map to `emerg` (0)   - Strings beginning with `a` (case-insensitive) map to `alert` (1)   - Strings beginning with `c` (case-insensitive) map to `critical` (2)   - Strings beginning with `err` (case-insensitive) map to `error` (3)   - Strings beginning with `w` (case-insensitive) map to `warning` (4)   - Strings beginning with `n` (case-insensitive) map to `notice` (5)   - Strings beginning with `i` (case-insensitive) map to `info` (6)   - Strings beginning with `d`, `trace` or `verbose` (case-insensitive) map to `debug` (7)   - Strings beginning with `o` or matching `OK` or `Success` (case-insensitive) map to OK   - All others map to `info` (6)    **Note:** If multiple log status remapper processors can be applied to a given log,   only the first one (according to the pipelines order) is taken into account.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsStatusRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    #[serde(rename = "type")]
    pub _type: crate::models::LogsStatusRemapperType,
}

impl LogsStatusRemapper {
    /// Use this Processor if you want to assign some attributes as the official status.  Each incoming status value is mapped as follows.    - Integers from 0 to 7 map to the Syslog severity standards   - Strings beginning with `emerg` or f (case-insensitive) map to `emerg` (0)   - Strings beginning with `a` (case-insensitive) map to `alert` (1)   - Strings beginning with `c` (case-insensitive) map to `critical` (2)   - Strings beginning with `err` (case-insensitive) map to `error` (3)   - Strings beginning with `w` (case-insensitive) map to `warning` (4)   - Strings beginning with `n` (case-insensitive) map to `notice` (5)   - Strings beginning with `i` (case-insensitive) map to `info` (6)   - Strings beginning with `d`, `trace` or `verbose` (case-insensitive) map to `debug` (7)   - Strings beginning with `o` or matching `OK` or `Success` (case-insensitive) map to OK   - All others map to `info` (6)    **Note:** If multiple log status remapper processors can be applied to a given log,   only the first one (according to the pipelines order) is taken into account.
    pub fn new(sources: Vec<String>, _type: crate::models::LogsStatusRemapperType) -> LogsStatusRemapper {
        LogsStatusRemapper {
            is_enabled: None,
            name: None,
            sources,
            _type,
        }
    }
}


