//! This module contains functionality for mapping colors.

use std::collections::HashMap;
use owo_colors::OwoColorize;

/// A type alias for a `HashMap` that maps color names to functions that colorize a string.
type ColorMap = HashMap<&'static str, Box<dyn Fn(&str) -> String>>;

/// Creates a color map that maps color names to functions that colorize a string.
///
/// # Returns
///
/// A `ColorMap`
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn initialize() -> ColorMap {
    let mut color_map: ColorMap = HashMap::new();
    color_map.insert("red", Box::new(|text| text.red().to_string()));
    color_map.insert("green", Box::new(|text| text.green().to_string()));
    color_map.insert("white", Box::new(|text| text.white().to_string()));
    color_map.insert("cyan", Box::new(|text| text.cyan().to_string()));
    color_map.insert("yellow", Box::new(|text| text.yellow().to_string()));

    #[cfg(feature = "tracing")]
    tracing::info!("COLOR_MAP initialized");

    color_map

}

/// Prints a message with a specific color and an optional icon prefix.
///
/// # Arguments
///
/// * `color_map`: A reference to a `HashMap` that maps color names to functions that colorize a string.
/// * `color`: A string slice that represents the color of the message and the prefix.
/// * `prefix`: A string slice that represents the icon to be printed before the message.
/// * `message`: A string slice that represents the message to be printed.
#[cfg_attr(feature = "tracing", tracing::instrument(skip(color_map)))]
pub fn display_message(color_map: &ColorMap, color: &str, prefix: &str, message: &str) {
    if let Some(color_fn) = color_map.get(color) {
        println!("{}{}", color_fn(prefix), color_fn(message));
        #[cfg(feature = "tracing")]
        tracing::info!("Message displayed with color: {}", color);
    } else {
        println!("{prefix}{message}");
        #[cfg(feature = "tracing")]
        tracing::error!("Failed to get color function for color: {}", color);
    }
}