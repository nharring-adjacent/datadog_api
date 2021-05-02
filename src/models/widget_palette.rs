/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// WidgetPalette : Color palette to apply.

/// Color palette to apply.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetPalette {
    #[serde(rename = "blue")]
    BLUE,
    #[serde(rename = "custom_bg")]
    CUSTOM_BACKGROUND,
    #[serde(rename = "custom_image")]
    CUSTOM_IMAGE,
    #[serde(rename = "custom_text")]
    CUSTOM_TEXT,
    #[serde(rename = "gray_on_white")]
    GRAY_ON_WHITE,
    #[serde(rename = "grey")]
    GREY,
    #[serde(rename = "green")]
    GREEN,
    #[serde(rename = "orange")]
    ORANGE,
    #[serde(rename = "red")]
    RED,
    #[serde(rename = "red_on_white")]
    RED_ON_WHITE,
    #[serde(rename = "white_on_gray")]
    WHITE_ON_GRAY,
    #[serde(rename = "white_on_green")]
    WHITE_ON_GREEN,
    #[serde(rename = "green_on_white")]
    GREEN_ON_WHITE,
    #[serde(rename = "white_on_red")]
    WHITE_ON_RED,
    #[serde(rename = "white_on_yellow")]
    WHITE_ON_YELLOW,
    #[serde(rename = "yellow_on_white")]
    YELLOW_ON_WHITE,
    #[serde(rename = "black_on_light_yellow")]
    BlackOnLightYellow,
    #[serde(rename = "black_on_light_green")]
    BlackOnLightGreen,
    #[serde(rename = "black_on_light_red")]
    BlackOnLightRed,

}

impl ToString for WidgetPalette {
    fn to_string(&self) -> String {
        match self {
            Self::BLUE => String::from("blue"),
            Self::CUSTOM_BACKGROUND => String::from("custom_bg"),
            Self::CUSTOM_IMAGE => String::from("custom_image"),
            Self::CUSTOM_TEXT => String::from("custom_text"),
            Self::GRAY_ON_WHITE => String::from("gray_on_white"),
            Self::GREY => String::from("grey"),
            Self::GREEN => String::from("green"),
            Self::ORANGE => String::from("orange"),
            Self::RED => String::from("red"),
            Self::RED_ON_WHITE => String::from("red_on_white"),
            Self::WHITE_ON_GRAY => String::from("white_on_gray"),
            Self::WHITE_ON_GREEN => String::from("white_on_green"),
            Self::GREEN_ON_WHITE => String::from("green_on_white"),
            Self::WHITE_ON_RED => String::from("white_on_red"),
            Self::WHITE_ON_YELLOW => String::from("white_on_yellow"),
            Self::YELLOW_ON_WHITE => String::from("yellow_on_white"),
            Self::BlackOnLightYellow => String::from("black_on_light_yellow"),
            Self::BlackOnLightGreen => String::from("black_on_light_green"),
            Self::BlackOnLightRed => String::from("black_on_light_red"),
        }
    }
}



