//! The `color_map` module provides functionality for mapping color names to their corresponding color functions.
//! It contains a `Lazy` static `COLOR_MAP` which is a thread-safe `HashMap` that maps color names to color functions.
//!
//! The `COLOR_MAP` is used to apply color to text based on the color name.
//! The `COLOR_MAP` is lazily initialized and contains mappings for the colors "red", "green", "white", "cyan", and "yellow".
//!
use std::sync::{Arc, Mutex};
use owo_colors::OwoColorize;
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// `ColorFunction` is a type alias for a thread-safe function pointer that takes a string slice and returns a colored string.
/// The function is wrapped in an `Arc<Mutex<_>>` to allow it to be shared and mutated across threads.
type ColorFunction = Arc<Mutex<dyn Fn(&str) -> String + Send>>;

/// `ColorMapType` is a type alias for a `HashMap` where the key is a static string slice representing the color name,
/// and the value is a `ColorFunction` which applies the corresponding color to a given string.
type ColorMapType = HashMap<&'static str, ColorFunction>;

/// A static `COLOR_MAP` that maps color names to color functions.
pub static COLOR_MAP: Lazy<ColorMapType> = Lazy::new(|| {
    let mut map: ColorMapType = HashMap::new();
    map.insert("red", Arc::new(Mutex::new(|text: &str | text.red().to_string())));
    map.insert("green", Arc::new(Mutex::new(|text:&str| text.green().to_string())));
    map.insert("white", Arc::new(Mutex::new(|text:&str| text.white().to_string())));
    map.insert("cyan", Arc::new(Mutex::new(|text:&str| text.cyan().to_string())));
    map.insert("yellow", Arc::new(Mutex::new(|text:&str| text.yellow().to_string())));

    map
});

#[cfg(test)]
mod color_map_tests {
    use super::*;

    #[test]
    fn color_map_contains_expected_colors() {
        let expected_colors = vec!["red", "green", "white", "cyan", "yellow"];
        for color in expected_colors {
            assert!(COLOR_MAP.contains_key(color));
        }
    }

    #[test]
    fn color_map_applies_correct_color() {
        let red_text = (COLOR_MAP.get("red").unwrap().lock().unwrap())("test");
        assert_eq!(red_text, "test".red().to_string());
    }

    #[test]
    fn color_map_returns_none_for_unknown_color() {
        assert!(COLOR_MAP.get("unknown_color").is_none());
    }

    #[test]
    fn color_map_handles_empty_string() {
        let red_text = (COLOR_MAP.get("red").unwrap().lock().unwrap())("");
        assert_eq!(red_text, "".red().to_string());
    }
}