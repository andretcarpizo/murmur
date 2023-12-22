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
/// ```
///
/// You must have [NerdFonts](https://www.nerdfonts.com/) installed to use the `Nf` variants.
/// - [Nerfonts github](https://github.com/ryanoasis/nerd-fonts?tab=readme-ov-files)
/// - [NerdFonts cheat-sheet](https://www.nerdfonts.com/cheat-sheet)
#[derive(Debug, Clone, Eq, PartialEq, Hash, Sequence)]
pub enum IconKind {
    NfFaTimes,
    NfFaCheck,
    NfFaInfoCircle,
    NfFaRefresh,
    NfFaWarning,
    NfFaBug,
    NfFaQuestion,
    NfFaQuestionCircle,
    NfFaTerminal,
    NfFaTrash,
    NfFaAngleRight,
    NfFaAngleLeft,
    NfFaAngleUp,
    NfFaAngleDown,
    NfFaThumbsUp,
    NfFaThumbsDown,
    NfFaeCcCc,

    NfOctDotFill,

    NfMdGreaterThan,
    NfMdLessThan,
    NfMdEqual,
    NfMdThumbsUp,
    NfMdThumbsDown,
    NfMdFolder,
    NfMdFolderOpen,

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

/// Red color.
const RED: &str = "red";
/// Green color.
const GREEN: &str = "green";
/// White color.
const WHITE: &str = "white";
/// Yellow color.
const YELLOW: &str = "yellow";
/// Cyan color.
const CYAN: &str = "cyan";

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
        let mut i_map = HashMap::new();
        // Nerd Font icons
        i_map.insert(IconKind::NfFaTimes, ("\u{f00d} ", RED)); // 
        i_map.insert(IconKind::NfFaCheck, ("\u{f00c} ", GREEN)); // 
        i_map.insert(IconKind::NfFaInfoCircle, ("\u{f05a} ", WHITE)); // 
        i_map.insert(IconKind::NfFaRefresh, ("\u{f021} ", CYAN)); // 
        i_map.insert(IconKind::NfFaWarning, ("\u{f071} ", YELLOW)); // 
        i_map.insert(IconKind::NfFaBug, ("\u{f188} ", RED)); // 
        i_map.insert(IconKind::NfFaQuestion, ("\u{f128} ", RED)); // 
        i_map.insert(IconKind::NfFaQuestionCircle, ("\u{f059} ", RED)); // 
        i_map.insert(IconKind::NfFaTerminal, ("\u{f120} ", WHITE)); // 
        i_map.insert(IconKind::NfFaTrash, ("\u{f1f8} ", WHITE)); // 
        i_map.insert(IconKind::NfFaAngleRight, ("\u{f105} ", WHITE)); // 
        i_map.insert(IconKind::NfFaAngleLeft, ("\u{f104} ", WHITE)); // 
        i_map.insert(IconKind::NfFaAngleUp, ("\u{f106} ", WHITE)); // 
        i_map.insert(IconKind::NfFaAngleDown, ("\u{f107} ", WHITE)); // 
        i_map.insert(IconKind::NfFaThumbsUp, ("\u{f164} ", GREEN)); // 
        i_map.insert(IconKind::NfFaThumbsDown, ("\u{f165} ", RED)); // 
        i_map.insert(IconKind::NfFaeCcCc, ("\u{e291} ", WHITE)); // 

        i_map.insert(IconKind::NfOctDotFill, ("\u{f444} ", WHITE)); // 

        i_map.insert(IconKind::NfMdGreaterThan, ("\u{f096d} ", WHITE)); // 󰥭
        i_map.insert(IconKind::NfMdLessThan, ("\u{f097c} ", WHITE)); // 󰥼
        i_map.insert(IconKind::NfMdEqual, ("\u{f01fc} ", WHITE)); // 󰇼
        i_map.insert(IconKind::NfMdThumbsUp, ("\u{f0513} ", GREEN)); // 󰔓
        i_map.insert(IconKind::NfMdThumbsDown, ("\u{f0511} ", RED)); // 󰔑
        i_map.insert(IconKind::NfMdFolder, ("\u{f024b} ", WHITE)); // 󰉋
        i_map.insert(IconKind::NfMdFolderOpen, ("\u{f0770} ", WHITE)); // 󰝰

        // Unicode icons
        #[rustfmt::skip]
        i_map.insert(IconKind::UnicodeInformationSource, ("\u{2139}\u{fe0f} ", WHITE)); // ℹ️
        i_map.insert(IconKind::UnicodeGear, ("\u{2699}\u{FE0F} ", CYAN)); // ⚙️
        i_map.insert(IconKind::UnicodeWarningSign, ("\u{26A0}\u{FE0F} ", YELLOW)); // ⚠️
        i_map.insert(IconKind::UnicodeBug, ("\u{1F41B} ", RED)); // 🐛
        i_map.insert(IconKind::UnicodeCrossMark, ("\u{274C} ", RED)); // ❌
        i_map.insert(IconKind::UnicodeCheckMark, ("\u{2714}\u{FE0F} ", GREEN)); // ✔️

        Mutex::new(i_map)
    });

#[cfg(test)]
mod icon_map_tests {
    use super::*;
    use crate::Whisper;
    use color_eyre::Report;
    use enum_iterator::all;

    #[test]
    fn test_color_eyre_install_setup() -> Result<(), Report> {
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

    #[test]
    fn test_whisper_all_icons() {
        all::<IconKind>()
            .collect::<Vec<_>>()
            .iter()
            .for_each(|icon_kind| {
                #[rustfmt::skip]
                Whisper::new()
                    .icon(icon_kind.clone())
                    .message(format!("{}: {}", icon_kind, ICON_MAP.lock().unwrap().get(icon_kind).unwrap().0))
                    .whisper()
                    .unwrap();
            });
    }

    /// This test function checks the spacing after each icon in the `ICON_MAP`.
    ///
    /// It iterates over each `IconKind` and its associated icon in the `ICON_MAP`.
    /// For each icon, it asserts that the icon ends with exactly one space.
    /// If an icon ends with no space or more than one space, the assertion fails and the test function panics.
    ///
    /// # Panics
    ///
    /// This function will panic if any icon in the `ICON_MAP` does not end with exactly one space.
    #[test]
    fn test_spaces_after_icons() {
        let icon_map = {
            let guard = ICON_MAP.lock().unwrap();
            guard.clone()
        };

        for (icon_kind, (icon, _)) in &icon_map {
            // Check that there is only one space after the icon
            assert!(
                icon.ends_with(' ') && !icon.ends_with("  "),
                "Invalid spacing after {icon_kind} icon: '{icon}'",
            );
        }
    }
}
