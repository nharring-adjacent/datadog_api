/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// DowntimeRecurrence : An object defining the recurrence of the downtime.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeRecurrence {
    /// How often to repeat as an integer. For example, to repeat every 3 days, select a type of `days` and a period of `3`.
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// The `RRULE` standard for defining recurring events (**requires to set \"type\" to rrule**) For example, to have a recurring event on the first day of each month, set the type to `rrule` and set the `FREQ` to `MONTHLY` and `BYMONTHDAY` to `1`. Most common `rrule` options from the [iCalendar Spec](https://tools.ietf.org/html/rfc5545) are supported.  **Note**: Attributes specifying the duration in `RRULE` are not supported (for example, `DTSTART`, `DTEND`, `DURATION`). More examples available in this [downtime guide](https://docs.datadoghq.com/monitors/guide/supress-alert-with-downtimes/?tab=api)
    #[serde(rename = "rrule", skip_serializing_if = "Option::is_none")]
    pub rrule: Option<String>,
    /// The type of recurrence. Choose from `days`, `weeks`, `months`, `years`, `rrule`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The date at which the recurrence should end as a POSIX timestamp. `until_occurences` and `until_date` are mutually exclusive.
    #[serde(rename = "until_date", skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    /// How many times the downtime is rescheduled. `until_occurences` and `until_date` are mutually exclusive.
    #[serde(rename = "until_occurrences", skip_serializing_if = "Option::is_none")]
    pub until_occurrences: Option<i32>,
    /// A list of week days to repeat on. Choose from `Mon`, `Tue`, `Wed`, `Thu`, `Fri`, `Sat` or `Sun`. Only applicable when type is weeks. First letter must be capitalized.
    #[serde(rename = "week_days", skip_serializing_if = "Option::is_none")]
    pub week_days: Option<Vec<String>>,
}

impl DowntimeRecurrence {
    /// An object defining the recurrence of the downtime.
    pub fn new() -> DowntimeRecurrence {
        DowntimeRecurrence {
            period: None,
            rrule: None,
            _type: None,
            until_date: None,
            until_occurrences: None,
            week_days: None,
        }
    }
}

