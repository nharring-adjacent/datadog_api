/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// WidgetLiveSpan : The available timeframes depend on the widget you are using.

/// The available timeframes depend on the widget you are using.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetLiveSpan {
    #[serde(rename = "1m")]
    PAST_ONE_MINUTE,
    #[serde(rename = "5m")]
    PAST_FIVE_MINUTES,
    #[serde(rename = "10m")]
    PAST_TEN_MINUTES,
    #[serde(rename = "15m")]
    PAST_FIFTEEN_MINUTES,
    #[serde(rename = "30m")]
    PAST_THIRTY_MINUTES,
    #[serde(rename = "1h")]
    PAST_ONE_HOUR,
    #[serde(rename = "4h")]
    PAST_FOUR_HOURS,
    #[serde(rename = "1d")]
    PAST_ONE_DAY,
    #[serde(rename = "2d")]
    PAST_TWO_DAYS,
    #[serde(rename = "1w")]
    PAST_ONE_WEEK,
    #[serde(rename = "1mo")]
    PAST_ONE_MONTH,
    #[serde(rename = "3mo")]
    PAST_THREE_MONTHS,
    #[serde(rename = "6mo")]
    PAST_SIX_MONTHS,
    #[serde(rename = "1y")]
    PAST_ONE_YEAR,
    #[serde(rename = "alert")]
    ALERT,

}

impl ToString for WidgetLiveSpan {
    fn to_string(&self) -> String {
        match self {
            Self::PAST_ONE_MINUTE => String::from("1m"),
            Self::PAST_FIVE_MINUTES => String::from("5m"),
            Self::PAST_TEN_MINUTES => String::from("10m"),
            Self::PAST_FIFTEEN_MINUTES => String::from("15m"),
            Self::PAST_THIRTY_MINUTES => String::from("30m"),
            Self::PAST_ONE_HOUR => String::from("1h"),
            Self::PAST_FOUR_HOURS => String::from("4h"),
            Self::PAST_ONE_DAY => String::from("1d"),
            Self::PAST_TWO_DAYS => String::from("2d"),
            Self::PAST_ONE_WEEK => String::from("1w"),
            Self::PAST_ONE_MONTH => String::from("1mo"),
            Self::PAST_THREE_MONTHS => String::from("3mo"),
            Self::PAST_SIX_MONTHS => String::from("6mo"),
            Self::PAST_ONE_YEAR => String::from("1y"),
            Self::ALERT => String::from("alert"),
        }
    }
}




