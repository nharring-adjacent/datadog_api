# DowntimeRecurrence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**period** | Option<**i32**> | How often to repeat as an integer. For example, to repeat every 3 days, select a type of `days` and a period of `3`. | [optional]
**rrule** | Option<**String**> | The `RRULE` standard for defining recurring events (**requires to set \"type\" to rrule**) For example, to have a recurring event on the first day of each month, set the type to `rrule` and set the `FREQ` to `MONTHLY` and `BYMONTHDAY` to `1`. Most common `rrule` options from the [iCalendar Spec](https://tools.ietf.org/html/rfc5545) are supported.  **Note**: Attributes specifying the duration in `RRULE` are not supported (for example, `DTSTART`, `DTEND`, `DURATION`). More examples available in this [downtime guide](https://docs.datadoghq.com/monitors/guide/supress-alert-with-downtimes/?tab=api) | [optional]
**_type** | Option<**String**> | The type of recurrence. Choose from `days`, `weeks`, `months`, `years`, `rrule`. | [optional]
**until_date** | Option<**i64**> | The date at which the recurrence should end as a POSIX timestamp. `until_occurences` and `until_date` are mutually exclusive. | [optional]
**until_occurrences** | Option<**i32**> | How many times the downtime is rescheduled. `until_occurences` and `until_date` are mutually exclusive. | [optional]
**week_days** | Option<**Vec<String>**> | A list of week days to repeat on. Choose from `Mon`, `Tue`, `Wed`, `Thu`, `Fri`, `Sat` or `Sun`. Only applicable when type is weeks. First letter must be capitalized. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


