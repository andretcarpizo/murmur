//! The `color_map` module provides functionality for mapping color names to their corresponding color functions.
//! It contains a `Lazy` static `COLOR_MAP` which is a thread-safe `HashMap` that maps color names to color functions.
//!
//! The `COLOR_MAP` is used to apply color to text based on the color name.
//! The `COLOR_MAP` is lazily initialized and contains mappings for the colors "red", "green", "white", "cyan", and "yellow".
//!
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use std::collections::HashMap;

/// A type alias for a boxed function that takes a `&str` and returns a `String`.
type ColorFn = Box<dyn Fn(&str) -> String + Send + Sync>;

/// A type alias for a `HashMap` that maps color names to color functions.
type ColorMapType = HashMap<&'static str, ColorFn>;

/// A tuple type that represents a color.
type Color = (&'static str, fn(&str) -> String);

/// A `Lazy` static `HashMap` that maps color names to color functions.
const COLORS: [Color; 5] = [
    ("red", |text: &str| text.red().to_string()),
    ("green", |text: &str| text.green().to_string()),
    ("white", |text: &str| text.white().to_string()),
    ("cyan", |text: &str| text.cyan().to_string()),
    ("yellow", |text: &str| text.yellow().to_string()),
];

/// A `Lazy` static `HashMap` that maps color names to color functions.
#[rustfmt::skip]
pub static COLOR_MAP: Lazy<ColorMapType> = Lazy::new(|| {
    let mut map: ColorMapType = HashMap::new();
    for &color in &COLORS {
        map.insert(color.0, Box::new(color.1));
    }
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
        // let red_text = (COLOR_MAP.get("red").unwrap().lock().unwrap())("test");
        let red_text = (COLOR_MAP.get("red").unwrap())("test");
        assert_eq!(red_text, "test".red().to_string());
    }

    #[test]
    fn color_map_returns_none_for_unknown_color() {
        assert!(COLOR_MAP.get("unknown_color").is_none());
    }

    #[test]
    fn color_map_handles_empty_string() {
        // let red_text = (COLOR_MAP.get("red").unwrap().lock().unwrap())("");
        let red_text = (COLOR_MAP.get("red").unwrap())("");
        assert_eq!(red_text, "".red().to_string());
    }
}
