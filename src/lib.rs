//! This is the `murmur` crate. This crate provides functionality for formatting print messages with NerdFont or Unicode icons.
//! It includes the `Whisper` struct which represents a collection of messages with an optional icon.
//!
//! # Example
//!
//! ```
//! use murmur::Whisper;
//! use murmur::IconKind;
//!
//! let whisper = Whisper::new()
//!     .icon(IconKind::NerdFontInformation)
//!     .message("This is a message")
//!     .whisper();
//! ```
//!
//! For more details, please refer to the individual modules and struct documentation.
#![warn(
clippy::all,
clippy::restriction,
clippy::pedantic,
clippy::nursery,
clippy::cargo,
)]
#![allow(
clippy::shadow_reuse,
clippy::missing_inline_in_public_items,
clippy::single_call_fn,
clippy::implicit_return,
clippy::missing_const_for_fn,
)]

mod icon_map;
mod color_map;

pub use icon_map::IconKind;
use core::fmt::{Debug, Display};


/// Represents a collection of messages with an optional icon and message
///
/// # Fields
///
/// * `icon_kind` - An optional field that specifies the kind of icon to be displayed.
/// * `messages` - A vector of messages to be displayed.
///
/// # Example
///
/// ```
/// use murmur::Whisper;
/// use murmur::IconKind;
///
/// let whisper = Whisper::new()
///     .icon(IconKind::NerdFontError)
///     .message("This is a message")
///     .whisper();
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Whisper {
    /// An optional field that specifies the kind of icon to be displayed.
    icon_kind: Option<IconKind>,
    /// A vector of messages to be displayed.
    messages: Vec<String>,
}

impl Whisper {
    /// Creates a new `Whisper` instance.
    ///
    /// # Returns
    ///
    /// A new `Whisper` instance with no icon and an empty message vector.
    ///
    /// # Example
    ///
    /// ```
    /// use murmur::Whisper;
    ///
    /// let whisper = Whisper::new();
    /// ```
    #[must_use]
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn new() -> Self {
        Self {
            icon_kind: None,
            messages: Vec::new(),
        }
    }

    /// Adds an icon to the `Whisper` instance.
    ///
    /// # Arguments
    ///
    /// * `icon_kind`: The kind of icon to be displayed.
    ///
    /// # Returns
    ///
    /// A `Whisper` instance with the specified icon.
    ///
    /// # Examples
    ///
    /// ```
    /// use murmur::Whisper;
    /// use murmur::IconKind;
    ///
    /// let whisper = Whisper::new()
    ///     .icon(IconKind::NerdFontError)
    ///     .message("This is a message")
    ///     .whisper();
    /// ```
    #[must_use]
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn icon(mut self, icon_kind: IconKind) -> Self {
        self.icon_kind = Some(icon_kind);
        self
    }

    /// Adds a single message to the `Whisper` instance.
    ///
    /// # Arguments
    ///
    /// * `message`: The message to be added.
    ///
    /// # Returns
    ///
    /// A `Whisper` instance with the added message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use murmur::Whisper;
    ///
    /// let whisper = Whisper::new()
    ///     .message("This is a message")
    ///     .whisper();
    /// ```
    #[must_use]
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn message<T: Display + Debug>(mut self, message: T) -> Self {
        self.messages.push(message.to_string());
        self
    }

    /// Adds multiple messages to the `Whisper` instance.
    ///
    /// # Arguments
    ///
    /// * `messages`: A vector of messages to be added.
    ///
    /// # Returns
    ///
    /// A `Whisper` instance with the added messages.
    ///
    /// # Examples
    ///
    /// ```
    /// use murmur::Whisper;
    ///
    /// let whisper = Whisper::new()
    ///     .message_vec(vec!["This is a message", "This is another message"])
    ///     .whisper();
    /// ```
    #[must_use]
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn message_vec<T: Display + Debug >(mut self, messages: Vec<T>) -> Self {
        for message in messages {
            self.messages.push(message.to_string());
        }
        self
    }

    /// Builds the `Whisper` instance and prints the messages.
    /// It first tries to lock the `ICON_MAP` to safely access the global variable in a concurrent environment.
    /// If the lock is successfully acquired, it checks the `icon_kind` field of the `Whisper` instance.
    /// If `icon_kind` is `Some`, it tries to get the corresponding icon and color from the `icon_map`.
    /// If `icon_kind` is `None` or if the `icon_kind` does not exist in the `icon_map`, it defaults to an empty string for both `icon` and `color`.
    /// Finally, it prints the messages with the specified color and an optional icon prefix.
    ///
    /// # Returns
    ///
    /// A `Whisper` instance with the specified icon and messages.
    #[must_use]
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn whisper(&self) -> Self {
        // Try to lock the ICON_MAP for safe access in a concurrent environment
        let Ok(icon_map) = icon_map::ICON_MAP.lock() else {
            #[cfg(feature = "tracing")]
            tracing::warn!("Failed to acquire lock on ICON_MAP");
            return Self::default();
        };

        // Check the icon_kind field of the Whisper instance
        let (icon, color) = self.icon_kind.clone().map_or(("", ""), |icon_kind| {
            icon_map.get(&icon_kind).map_or(("", ""), |value| *value)
        });

        // Print the messages with the specified color and an optional icon prefix
        self.print_messages(icon, color);

        #[cfg(feature = "tracing")]
        tracing::info!("Printed messages with icon and color");

        // Return a clone of the Whisper instance
        self.clone()
    }

    /// Prints messages with a specific color and an optional icon prefix.
    ///
    /// # Arguments
    ///
    /// * `icon`: A string slice that represents the icon to be printed before each message.
    /// * `color`: A string slice that represents the color of the messages and the icon.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn print_messages(&self, icon: &str, color: &str) {
        let color_map = color_map::initialize();
        let messages = if self.messages.is_empty() {
            vec![String::new()]
        } else {
            self.messages.clone()
        };

        for (index, message) in messages.iter().enumerate() {
            let prefix = if index == 0 { icon } else { "  " };
            color_map::display_message(&color_map, color, prefix, message);
        }
    }
}

#[cfg(test)]
mod whisper_tests {
    use super::*;

    #[test]
    fn test_whisper_no_icon_no_messages() {
        // Test creating a Whisper instance with no icon and no messages
        let whisper_instance = Whisper::new().whisper();
        assert_eq!(whisper_instance.icon_kind, None);
        assert_eq!(whisper_instance.messages, Vec::<String>::new());
    }

    #[test]
    fn test_whisper_no_icon_one_message() {
        // Test creating a Whisper instance with no icon and one message
        let whisper_instance = Whisper::new().message("message without icon").whisper();
        assert_eq!(whisper_instance.icon_kind, None);
        assert_eq!(whisper_instance.messages, vec!["message without icon"]);
    }

    #[test]
    fn test_whisper_no_icon_multiple_messages() {
        // Test creating a Whisper instance with no icon and multiple messages
        let whisper_instance = Whisper::new()
            .message("1 message without icon")
            .message("2 message without icon")
            .message("3 message without icon")
            .whisper();
        assert_eq!(whisper_instance.icon_kind, None);
        assert_eq!(
            whisper_instance.messages.as_slice(),
            &[
                "1 message without icon",
                "2 message without icon",
                "3 message without icon"
            ]
        );
    }

    #[test]
    fn test_whisper_icon_no_message() {
        // Test creating a Whisper instance with an icon and no messages
        let whisper_instance = Whisper::new().icon(IconKind::NerdFontDebugging).whisper();
        assert_eq!(
            whisper_instance.icon_kind,
            Some(IconKind::NerdFontDebugging)
        );
        assert_eq!(whisper_instance.messages, Vec::<String>::new());
    }

    #[test]
    fn test_whisper_icon_message() {
        // Test creating a Whisper instance with an icon and a single message
        let whisper_instance = Whisper::new()
            .icon(IconKind::NerdFontInformation)
            .message("message with icon")
            .whisper();
        assert_eq!(
            whisper_instance.icon_kind,
            Some(IconKind::NerdFontInformation)
        );
        assert_eq!(whisper_instance.messages.as_slice(), &["message with icon"]);
    }

    #[test]
    fn test_whisper_icon_multiple_messages() {
        // Test creating a Whisper instance with an icon and multiple messages
        let whisper_instance = Whisper::new()
            .icon(IconKind::NerdFontWarning)
            .message("First message")
            .message("Second message")
            .message("Third message")
            .whisper();
        assert_eq!(whisper_instance.icon_kind, Some(IconKind::NerdFontWarning));
        assert_eq!(
            whisper_instance.messages.as_slice(),
            &["First message", "Second message", "Third message"]
        );
    }

    #[test]
    fn test_whisper_icon_multiple_messages_message_vec() {
        // Test creating a Whisper instance with an icon and multiple messages and message_vec
        let whisper_instance = Whisper::new()
            .icon(IconKind::NerdFontSuccess)
            .message("First message")
            .message("Second message")
            .message_vec(vec!["Third message", "Fourth message"])
            .whisper();
        assert_eq!(whisper_instance.icon_kind, Some(IconKind::NerdFontSuccess));
        assert_eq!(
            whisper_instance.messages.as_slice(),
            &[
                "First message",
                "Second message",
                "Third message",
                "Fourth message"
            ]
        );
    }

    #[test]
    fn test_whisper_icon_multiple_message_vec() {
        // Test creating a Whisper instance with an icon and multiple vec messages
        let whisper_instance = Whisper::new()
            .icon(IconKind::NerdFontWarning)
            .message_vec(vec!["Line", "Another line"])
            .message_vec(vec!["Another line"])
            .whisper();
        assert_eq!(
            whisper_instance.messages,
            vec!["Line", "Another line", "Another line"]
        );
    }

    #[test]
    fn test_icon_multiple_message_vec_message() {
        // Test creating a Whisper instance with an icon and multiple vec messages and message
        let whisper_instance = Whisper::new()
            .icon(IconKind::NerdFontWarning)
            .message_vec(vec!["Line", "Another line"])
            .message("Another line")
            .whisper();
        assert_eq!(
            whisper_instance.messages,
            vec!["Line", "Another line", "Another line"]
        );
    }

    #[test]
    fn test_message_vec_empty_messages() {
        // Test for the `message_vec` method when the `messages` vector is empty.
        let whisper_instance = Whisper::new().message_vec(Vec::<String>::new()).whisper();
        assert_eq!(whisper_instance.icon_kind, None);
        assert_eq!(whisper_instance.messages, Vec::<String>::new());
    }

    #[test]
    fn test_message_vec_multiple_messages() {
        // Test for the `message_vec` method when the `messages` vector contains multiple elements.
        let whisper_instance = Whisper::new()
            .message_vec(vec!["Test message 1", "Test message 2"])
            .whisper();
        assert_eq!(whisper_instance.icon_kind, None);
        assert_eq!(
            whisper_instance.messages,
            vec!["Test message 1", "Test message 2"]
        );
    }

    #[test]
    fn test_whisper_add_icon_random_order() {
        //Test adding icon in the middle of messages
        let whisper_instance = Whisper::new()
            .message("Test adding icon in random place")
            .icon(IconKind::NerdFontDebugging)
            .message("icon should be added to the first message")
            .whisper();
        assert_eq!(
            whisper_instance.icon_kind,
            Some(IconKind::NerdFontDebugging)
        );
        assert_eq!(
            whisper_instance.messages,
            vec![
                "Test adding icon in random place",
                "icon should be added to the first message"
            ]
        );
    }

    #[test]
    fn test_whisper_append_icon_message_to_instance() {
        // Test creating a Whisper instance and appending a message and icon after creation
        let whisper_instance =
            Whisper::new().message("Test creating a Whisper instance with message");
        assert_eq!(whisper_instance.icon_kind, None);
        assert_eq!(
            whisper_instance.messages,
            vec!["Test creating a Whisper instance with message"]
        );

        let whisper_instance = whisper_instance
            .message("Append a message and icon after creation")
            .icon(IconKind::NerdFontInformation)
            .whisper();
        assert_eq!(
            whisper_instance.icon_kind,
            Some(IconKind::NerdFontInformation)
        );
        assert_eq!(
            whisper_instance.messages,
            vec![
                "Test creating a Whisper instance with message",
                "Append a message and icon after creation"
            ]
        );
    }

    #[test]
    fn test_whisper_default() {
        // Test default
        let whisper_instance = Whisper::default()
            .icon(IconKind::NerdFontProcessing)
            .message("Test default")
            .whisper();
        assert_eq!(
            whisper_instance.icon_kind,
            Some(IconKind::NerdFontProcessing)
        );
    }
}
