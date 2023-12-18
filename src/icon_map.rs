//! Module containing the icon map for formatting messages.

use enum_iterator::Sequence;
use once_cell::sync::Lazy;
use std::fmt;
use std::{collections::HashMap, sync::Mutex};

/// Enum representing different kinds of icons for formatting messages.
/// Unicode or Nerd Font icons if you have a Nerd Font installed.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Sequence)]
pub enum IconKind {
    NerdFontError,
    NerdFontSuccess,
    NerdFontInformation,
    NerdFontProcessing,
    NerdFontWarning,
    NerdFontDebugging,

    UnicodeError,
    UnicodeSuccess,
    UnicodeInformation,
    UnicodeProcessing,
    UnicodeWarning,
    UnicodeDebugging,
}

impl fmt::Display for IconKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// A static `ICON_MAP` that maps `IconKind` to a tuple of icon and color.
///
/// This map is lazily initialized and thread-safe. It contains mappings for both `NerdFont` and Unicode icons.
/// Each `IconKind` is mapped to a tuple, where the first element is the icon character and the second element is the color.
///
/// The `ICON_MAP` is used by the `Whisper` struct to look up the icon and color based on the `IconKind`.
///
/// If the `tracing` feature is enabled, an informational message will be logged when the `ICON_MAP` is initialized.
pub static ICON_MAP: Lazy<Mutex<HashMap<IconKind, (&'static str, &'static str)>>> =
    Lazy::new(|| {
        let mut icon_map = HashMap::new();
        // Nerd Font icons
        icon_map.insert(IconKind::NerdFontError, (" ", "red"));
        icon_map.insert(IconKind::NerdFontSuccess, (" ", "green"));
        icon_map.insert(IconKind::NerdFontInformation, (" ", "white"));
        icon_map.insert(IconKind::NerdFontProcessing, (" ", "cyan"));
        icon_map.insert(IconKind::NerdFontWarning, (" ", "yellow"));
        icon_map.insert(IconKind::NerdFontDebugging, (" ", "red"));

        // Unicode icons
        icon_map.insert(IconKind::UnicodeError, ("❌ ", "red"));
        icon_map.insert(IconKind::UnicodeSuccess, ("✔️ ", "green"));
        icon_map.insert(IconKind::UnicodeInformation, ("ℹ️ ", "white"));
        icon_map.insert(IconKind::UnicodeProcessing, ("⚙️ ", "cyan"));
        icon_map.insert(IconKind::UnicodeWarning, ("⚠️ ", "yellow"));
        icon_map.insert(IconKind::UnicodeDebugging, ("🐛 ", "yellow"));

        #[cfg(feature = "tracing")]
        tracing::info!("ICON_MAP initialized");

        Mutex::new(icon_map)
    });

#[cfg(test)]
mod icon_map_tests {
    use super::*;
    use enum_iterator::all;

    /// Test function to print all icons associated with different kinds of messages.
    ///
    /// The function begins by calling `all::<IconKind>().collect::<Vec<_>>()`.
    /// This line of code uses the `all` function from the `enum_iterator` crate
    /// to create an iterator over all variants of the `IconKind` enum.
    /// The `collect::<Vec<_>>()` function is then used to collect these variants into a vector.
    ///
    /// Next, the function calls `iter()` on the vector to create an iterator,
    /// and then uses `for_each` to apply a closure to each `IconKind` variant in the iterator.
    ///
    /// Inside the closure, the function first acquires a lock on the `ICON_MAP` static variable,
    /// which is a thread-safe `HashMap` that maps `IconKind` enum variants to a tuple of an icon and a color.
    /// The `unwrap()` function is used to handle the `Result` returned by `lock()`.
    /// If the lock can't be acquired, the program will panic and terminate.
    ///
    /// Then, the function calls `get(icon_kind)` on the `ICON_MAP` to look up the icon and color associated with the current `IconKind` variant.
    /// The `unwrap().0` at the end extracts the icon from the tuple (ignoring the color),
    /// and if the `get` call returns `None` (i.e., if the `IconKind` variant is not found in the `ICON_MAP`),
    /// the program will panic and terminate.
    ///
    /// Finally, the function prints the `IconKind` variant and the associated icon to the console.
    ///
    /// In summary, this test function is used to print all the icons in the `ICON_MAP` to the console.
    /// It's a simple way to visually check that all the icons are correctly mapped to their corresponding `IconKind` variants.
    #[test]
    fn test_print_all_icons() {
        all::<IconKind>()
            .collect::<Vec<_>>()
            .iter()
            .for_each(|icon_kind| {
                println!(
                    "{}: {}",
                    icon_kind,
                    ICON_MAP.lock().unwrap().get(icon_kind).unwrap().0
                );
            });
    }
}
