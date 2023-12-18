//! Module containing the icon map for formatting messages.

use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

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
