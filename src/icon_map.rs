//! The `icon_map` module provides functionality for mapping `IconKind` enum variants to their corresponding icons and colors.
//! It contains a `Lazy` static `ICON_MAP` which is a thread-safe `HashMap` that maps `IconKind` enum variants to a tuple of an icon and a color.
//! The `ICON_MAP` is used by the `Whisper` struct to look up the icon and color based on the `IconKind`.
//! The `ICON_MAP` is lazily initialized and contains mappings for both `NerdFont` and Unicode icons.
//!
//! The `IconKind` enum represents different kinds of icons for formatting messages. It supports both Unicode or Nerd Font icons if you have a Nerd Font installed.
//!
use enum_iterator::Sequence;
use once_cell::sync::Lazy;
use std::fmt;
use std::{collections::HashMap, sync::Mutex};

/// `IconKind` is an enum representing different kinds of icons for formatting messages.
/// Supports both `Unicode` and `NerdFonts` icons if you have a Nerd Font installed.
///
///  # Examples
/// ```
/// use murmur::{Whisper, IconKind};
///
/// Whisper::new()
///     .icon(IconKind::NfFaTimes)
///     .message("This is a message with a Nerd Font icon.")
///     .whisper()
///     .unwrap();
///
/// ```
///
/// [Nerd Fonts Cheat Sheet](https://www.nerdfonts.com/cheat-sheet)
#[derive(Debug, Clone, Eq, PartialEq, Hash, Sequence)]
pub enum IconKind {
    NfFaTimes,
    NfFaCheck,
    NfFaInfoCircle,
    NfFaRefresh,
    NfFaWarning,
    NfFaBug,

    UnicodeCrossMark,
    UnicodeCheckMark,
    UnicodeInformationSource,
    UnicodeGear,
    UnicodeWarningSign,
    UnicodeBug,
}

/// Implement the `Display` trait for `IconKind`.
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
        icon_map.insert(IconKind::NfFaTimes, ("\u{f00d} ", "red")); // 
        icon_map.insert(IconKind::NfFaCheck, ("\u{f00c} ", "green")); // 
        icon_map.insert(IconKind::NfFaInfoCircle, ("\u{f05a} ", "white")); // 
        icon_map.insert(IconKind::NfFaRefresh, ("\u{f021} ", "cyan")); // 
        icon_map.insert(IconKind::NfFaWarning, ("\u{f071} ", "yellow")); // 
        icon_map.insert(IconKind::NfFaBug, ("\u{f188} ", "red")); // 

        // Unicode icons
        icon_map.insert(IconKind::UnicodeCrossMark, ("\u{274C} ", "red")); // ❌
        icon_map.insert(IconKind::UnicodeCheckMark, ("\u{2714}\u{FE0F} ", "green")); // ✔️
        icon_map.insert(
            IconKind::UnicodeInformationSource,
            ("\u{2139}\u{FE0F} ", "white"),
        ); // ℹ️
        icon_map.insert(IconKind::UnicodeGear, ("\u{2699}\u{FE0F} ", "cyan")); // ⚙️
        icon_map.insert(
            IconKind::UnicodeWarningSign,
            ("\u{26A0}\u{FE0F} ", "yellow"),
        ); // ⚠️
        icon_map.insert(IconKind::UnicodeBug, ("\u{1F41B} ", "red")); // 🐛

        Mutex::new(icon_map)
    });

#[cfg(test)]
mod icon_map_tests {
    use super::*;
    use crate::Whisper;
    use color_eyre::Report;
    use enum_iterator::all;

    #[test]
    fn color_eyre_install_setup() -> Result<(), Report> {
        color_eyre::install()?;
        Whisper::new().message("color_eyre installed").whisper()?;
        Ok(())
    }
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
    fn test_print_all_icons() -> Result<(), Report> {
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
        Ok(())
    }

    #[test]
    fn test_spaces_after_icons() -> Result<(), Report> {
        for (icon_kind, (icon, _)) in ICON_MAP.lock().unwrap().iter() {
            // Check that there is only one space after the icon
            assert!(
                icon.ends_with(" ") && !icon.ends_with("  "),
                "Invalid spacing after {} icon: '{}'",
                icon_kind,
                icon
            );
        }
        Ok(())
    }
}
