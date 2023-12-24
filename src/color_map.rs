//! The `color_map` module provides functionality for mapping color names to their corresponding color functions.
//! It contains a `Lazy` static `COLOR_MAP` which is a thread-safe `HashMap` that maps color names to color functions.
//!
//! The `COLOR_MAP` is used to apply color to text based on the color name.
//! The `COLOR_MAP` is lazily initialized and contains mappings for the colors "red", "green", "white", "cyan", and "yellow".
//!
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use std::collections::HashMap;

/// `ColorFunction` is a type alias for a trait object that represents a function which takes a string slice as input
/// and returns a string. This function is used to apply a color to the given string.
///
/// The `Box<dyn Fn(&str) -> String + Send + Sync>` syntax indicates that the trait object is owned by a `Box`,
/// and it can be dynamically dispatched to any type that implements `Fn(&str) -> String` trait. The `Send` and `Sync`
/// bounds ensure that the trait object can be safely sent between threads and shared across them.
///
/// Example usage:
/// ```
/// use std::boxed::Box;
///
/// type ColorFunction = Box<dyn Fn(&str) -> String + Send + Sync>;
///
/// fn apply_color(color_fn: &ColorFunction, text: &str) -> String {
///     color_fn(text)
/// }
///
/// fn main() {
///     let color_fn: ColorFunction = Box::new(|text| format!("\x1B[31m{}\x1B[0m", text));
///     let colored_text = apply_color(&color_fn, "Hello world!");
///     println!("{}", colored_text);
/// }
/// ```
/// In this example, `ColorFunction` is used to represent a function that applies red color to a given string. The function
/// `apply_color` takes a reference to a `ColorFunction` and a string slice, and applies the color function to the text,
/// returning the colored string. Finally, the colored text is printed to the console.
///
type ColorFunction = Box<dyn Fn(&str) -> String + Send + Sync>;

/// `ColorMapType` is a type alias for a `HashMap` where the key is a static string slice representing the color name,
/// and the value is a `ColorFunction` which applies the corresponding color to a given string.
type ColorMapType = HashMap<&'static str, ColorFunction>;

/// Creates a `ColorFunction` from a closure.
///
/// The closure `f` takes a reference to a `str` and returns a `String`.
/// The closure must be `'static`, meaning it has no references to local data.
/// The closure must be both `Send` and `Sync`, allowing it to be safely shared
/// between multiple threads.
///
/// # Arguments
///
/// * `f` - The closure that transforms a `str` into a `String`.
///
/// # Returns
///
/// A `ColorFunction` that wraps the provided closure.
fn create_color<F>(f: F) -> ColorFunction
where
    F: 'static + Fn(&str) -> String + Send + Sync,
{
    // Arc::new(Mutex::new(f))
    Box::new(f)
}

/// A static `COLOR_MAP` that maps color names to color functions.
#[rustfmt::skip]
pub static COLOR_MAP: Lazy<ColorMapType> = Lazy::new(|| {
    let mut map: ColorMapType = HashMap::new();
    map.insert("red", create_color(|text: &str| text.red().to_string()));
    map.insert("green", create_color(|text: &str| text.green().to_string()));
    map.insert("white", create_color(|text: &str| text.white().to_string()));
    map.insert("cyan", create_color(|text: &str| text.cyan().to_string()));
    map.insert("yellow", create_color(|text: &str| text.yellow().to_string()));
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
