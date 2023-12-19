//! This module contains functionality to print messages to stdout based on a color hash map.

use owo_colors::OwoColorize;
use std::collections::HashMap;
use once_cell::sync::Lazy;

type ColorFunction = Box<dyn Fn(&str) -> String>;
type ColorMapType = HashMap<&'static str, ColorFunction>;

pub static COLOR_MAP: Lazy<ColorMapType> = Lazy::new(|| {
    let mut map: ColorMapType = HashMap::new();
    map.insert("red", Box::new(|text| text.red().to_string()));
    map.insert("green", Box::new(|text| text.green().to_string()));
    map.insert("white", Box::new(|text| text.white().to_string()));
    map.insert("cyan", Box::new(|text| text.cyan().to_string()));
    map.insert("yellow", Box::new(|text| text.yellow().to_string()));

    map
});

#[cfg(test)]
mod color_map_tests {
    use super::*;

    #[test]
    fn color_map_initializes_with_correct_colors() {
        let color_map = ColorMap::new();

        assert_eq!(color_map.map.get("red").is_some(), true);
        assert_eq!(color_map.map.get("green").is_some(), true);
        assert_eq!(color_map.map.get("white").is_some(), true);
        assert_eq!(color_map.map.get("cyan").is_some(), true);
        assert_eq!(color_map.map.get("yellow").is_some(), true);
    }

    #[test]
    fn color_map_returns_none_for_nonexistent_color() {
        let color_map = ColorMap::new();

        assert_eq!(color_map.map.get("purple").is_some(), false);
    }

    #[test]
    fn color_map_applies_correct_color_function() {
        let color_map = ColorMap::new();
        let red_function = color_map.map.get("red").unwrap();

        assert_eq!(red_function("test"), "test".red().to_string());
    }

    #[test]
    fn color_map_default_initializes_empty_map() {
        let color_map = ColorMap::default();

        assert_eq!(color_map.map.is_empty(), true);
    }
}