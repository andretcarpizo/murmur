//! Module containing the icon map for formatting messages.

use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};
use std::fmt;

/// Enum representing different kinds of icons for formatting messages.
/// Unicode or Nerd Font icons if you have a Nerd Font installed.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
        write!(f, "{:?}", self)
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
mod tests {
    use super::*;

    #[test]
    fn test_print_nerd_font_icons() {
        let icon_map = ICON_MAP.lock().unwrap();

        for (icon_kind, (icon, _color)) in icon_map.iter() {
            match icon_kind {
                IconKind::NerdFontError
                | IconKind::NerdFontSuccess
                | IconKind::NerdFontInformation
                | IconKind::NerdFontProcessing
                | IconKind::NerdFontWarning
                | IconKind::NerdFontDebugging => {
                    println!("{}: {}", icon_kind, icon);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_print_unicode_icons() {
        let icon_map = ICON_MAP.lock().unwrap();

        for (icon_kind, (icon, _color)) in icon_map.iter() {
            match icon_kind {
                IconKind::UnicodeError
                | IconKind::UnicodeSuccess
                | IconKind::UnicodeInformation
                | IconKind::UnicodeProcessing
                | IconKind::UnicodeWarning
                | IconKind::UnicodeDebugging => {
                    println!("{}: {}", icon_kind, icon);
                }
                _ => {}
            }
        }
    }
}

