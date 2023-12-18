//! This crate provides functionality for formatting print messages with `NerdFont` or `Unicode` icons.
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

mod icon_map;
pub use icon_map::IconKind;

mod color_map;
use color_map::ColorMap;

mod murmuro;
use murmuro::murmur_message;

use core::fmt::{Debug, Display};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MurmurError {
    #[error("Failed to acquire lock on ICON_MAP")]
    LockError,
    #[error("Failed to print message")]
    PrintError,
}

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
///     .icon(IconKind::NerdFontInformation)
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
    pub fn message_vec<T: Display + Debug>(mut self, messages: Vec<T>) -> Self {
        for message in messages {
            self.messages.push(message.to_string());
        }
        self
    }

    /// The `whisper` function is responsible for building the `Whisper` instance and printing the messages.
    /// It performs several steps to ensure the messages are printed correctly:
    ///
    /// 1. It first tries to lock the `ICON_MAP` to safely access the global variable in a concurrent environment.
    /// 2. If the lock is successfully acquired, it checks the `icon_kind` field of the `Whisper` instance.
    /// 3. If `icon_kind` is `Some`, it tries to get the corresponding icon and color from the `icon_map`.
    /// 4. If `icon_kind` is `None` or if the `icon_kind` does not exist in the `icon_map`, it defaults to an empty string for both `icon` and `color`.
    /// 5. Finally, it prints the messages with the specified color and an optional icon prefix.
    ///
    /// # Returns
    ///
    /// This function returns a `Result`. If the operation is successful, it returns `Ok(())`. If there is an error during the operation, it returns `Err(MurmurError)`.
    ///
    /// # Errors
    ///
    /// This function will return `Err(MurmurError::LockError)` if it fails to acquire a lock on the `ICON_MAP`.
    /// It will return `Err(MurmurError::PrintError)` if there is an error while printing the messages.
    ///
    /// # Example
    ///
    /// ```rust
    /// use murmur::Whisper;
    /// use murmur::IconKind;
    ///
    /// let whisper = Whisper::new()
    ///     .icon(IconKind::NerdFontInformation)
    ///     .message("This is a message");
    ///
    /// match whisper.whisper() {
    ///     Ok(_) => println!("Message printed successfully"),
    ///     Err(e) => eprintln!("Failed to print message: {}", e),
    /// }
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn whisper(&self) -> Result<(), MurmurError> {
        // Try to lock the ICON_MAP for safe access in a concurrent environment
        let icon_map = match icon_map::ICON_MAP.lock() {
            Ok(icon_map) => icon_map,
            Err(err) => {
                eprintln!("Error: {}", &err);
                #[cfg(feature = "tracing")]
                tracing::warn!("Error: {}", &err);
                return Err(MurmurError::LockError);
            }
        };

        // Check the icon_kind field of the Whisper instance
        let (icon, color) = self.icon_kind.clone().map_or(("", ""), |icon_kind| {
            icon_map.get(&icon_kind).map_or(("", ""), |value| *value)
        });

        // Print the messages with the specified color and an optional icon prefix
       self.murmur_messages(icon, color).map_err(|err| {
            eprintln!("Error: {}", &err);
            #[cfg(feature = "tracing")]
            tracing::error!("Error: {}", &err);
            MurmurError::PrintError
        })?;


        #[cfg(feature = "tracing")]
        tracing::info!("Printed messages with icon and color");

        // Return Ok to indicate successful operation
        Ok(())
    }

    /// Prints messages with a specific color and an optional icon prefix.
    ///
    /// This function is responsible for printing each message in the `Whisper` instance with a specific color and an optional icon prefix.
    /// It first creates a new `ColorMap` instance to map colors to their corresponding formatting functions.
    /// Then, it checks if the `messages` vector of the `Whisper` instance is empty. If it is, it creates a new vector with an empty string.
    /// Otherwise, it clones the `messages` vector.
    ///
    /// For each message in the `messages` vector, it determines the prefix. If the message is the first in the vector, the prefix is the `icon`.
    /// For all other messages, the prefix is two spaces.
    ///
    /// Finally, it calls the `murmur_message` function to print each message with the specified color and prefix.
    /// If there is an error while printing the messages, it returns `Err(MurmurError::PrintError)`.
    ///
    /// # Arguments
    ///
    /// * `icon`: A string slice that represents the icon to be printed before each message.
    /// * `color`: A string slice that represents the color of the messages and the icon.
    ///
    /// # Returns
    ///
    /// This function returns a `Result`. If the operation is successful, it returns `Ok(())`. If there is an error during the operation, it returns `Err(MurmurError)`.
    ///
    /// # Errors
    ///
    /// This function will return `Err(MurmurError::PrintError)` if there is an error while printing the messages.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn murmur_messages(&self, icon: &str, color: &str) -> Result<(), MurmurError> {
        let color_map = ColorMap::new();
        let messages = if self.messages.is_empty() {
            vec![String::new()]
        } else {
            self.messages.clone()
        };

        for (index, message) in messages.iter().enumerate() {
            let prefix = if index == 0 { icon } else { "  " };
            murmur_message(&color_map, color, prefix, message).map_err(|err| {
                eprintln!("Error: {}", &err);
                #[cfg(feature = "tracing")]
                tracing::error!("Error: {}", &err);
                MurmurError::PrintError
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod whisper_tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_whisper_propagate_to_murmur_error() -> Result<(), MurmurError> {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_propagate_to_murmur_error")
            .whisper()?;
        Ok(())
    }

    #[test]
    fn test_whisper_unwrap()  {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_unwrap")
            .whisper()
            .unwrap();
    }

    #[test]
    fn test_whisper_map_err_and_propagate_io_error() -> Result<(), Error> {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_user_api_map_err_and_propagate_io_error")
            .whisper()
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        Ok(())
    }

    #[test]
    fn test_whisper_match() {
        let whisper = Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_match");

        match whisper.whisper() {
            Ok(_) => println!("Message printed successfully"),
            Err(error) => eprintln!("Failed to print message: {}", error),
        }
    }

    #[test]
    fn test_whisper_no_icon_no_messages() {
        // Test creating a Whisper instance with no icon and no messages
        let whisper = Whisper::new();
        let result = whisper.whisper();
        assert!(result.is_ok()); // Check that whisper did not return an error
        assert_eq!(whisper.icon_kind, None);
        assert_eq!(whisper.messages, Vec::<String>::new());
    }

    #[test]
    fn test_whisper_no_icon_one_message() {
        let whisper = Whisper::new().message("message without icon");
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, None);
        assert_eq!(whisper.messages, vec!["message without icon"]);
    }

    #[test]
    fn test_whisper_no_icon_multiple_messages() {
        let whisper = Whisper::new()
            .message("1 message without icon")
            .message("2 message without icon")
            .message("3 message without icon");
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, None);
        assert_eq!(
            whisper.messages.as_slice(),
            &[
                "1 message without icon",
                "2 message without icon",
                "3 message without icon"
            ]
        );
    }

    #[test]
    fn test_whisper_icon_no_message() {
        // After
        let whisper = Whisper::new().icon(IconKind::NerdFontDebugging);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NerdFontDebugging));
        assert_eq!(whisper.messages, Vec::<String>::new());
    }

    #[test]
    fn test_whisper_icon_message() {
        // After
        let whisper = Whisper::new()
            .icon(IconKind::NerdFontInformation)
            .message("message with icon");
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NerdFontInformation));
        assert_eq!(whisper.messages.as_slice(), &["message with icon"]);
    }

    #[test]
    fn test_whisper_icon_multiple_messages() {
        // After
        let whisper = Whisper::new()
            .icon(IconKind::NerdFontWarning)
            .message("First message")
            .message("Second message")
            .message("Third message");
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NerdFontWarning));
        assert_eq!(
            whisper.messages.as_slice(),
            &["First message", "Second message", "Third message"]
        );
    }

    #[test]
    fn test_whisper_icon_multiple_messages_message_vec() {
        // After
        let whisper = Whisper::new()
            .icon(IconKind::NerdFontSuccess)
            .message("First message")
            .message("Second message")
            .message_vec(vec!["Third message", "Fourth message"]);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NerdFontSuccess));
        assert_eq!(
            whisper.messages.as_slice(),
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
        let whisper = Whisper::new()
            .icon(IconKind::NerdFontWarning)
            .message_vec(vec!["Line", "Another line"])
            .message_vec(vec!["Another line"]);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(
            whisper.messages,
            vec!["Line", "Another line", "Another line"]
        );
    }

    #[test]
    fn test_icon_multiple_message_vec_message() {
        let whisper = Whisper::new()
            .icon(IconKind::NerdFontWarning)
            .message_vec(vec!["Line", "Another line"])
            .message_vec(vec!["Another line"]);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(
            whisper.messages,
            vec!["Line", "Another line", "Another line"]
        );
    }
    #[test]
    fn test_message_vec_empty_messages() {
        // Test for the `message_vec` method when the `messages` vector is empty.
        let whisper = Whisper::new().message_vec(Vec::<String>::new());
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, None);
        assert_eq!(whisper.messages, Vec::<String>::new());
    }

    #[test]
    fn test_message_vec_multiple_messages() {
        // Test for the `message_vec` method when the `messages` vector contains multiple elements.
        let whisper = Whisper::new()
            .icon(IconKind::NerdFontError)
            .message_vec(vec!["Test message vec 1", "Test message vec 2"]);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NerdFontError));
        assert_eq!(
            whisper.messages,
            vec!["Test message vec 1", "Test message vec 2"]
        );
    }

    #[test]
    fn test_whisper_add_icon_random_order() {
        //Test adding icon in the middle of messages
        let whisper = Whisper::new()
            .message("Test adding icon in random place")
            .icon(IconKind::NerdFontDebugging)
            .message("icon should be added to the first message");
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NerdFontDebugging));
        assert_eq!(
            whisper.messages,
            vec![
                "Test adding icon in random place",
                "icon should be added to the first message"
            ]
        );
    }

    #[test]
    fn test_whisper_append_icon_message_to_instance() {
        // Test creating a Whisper instance and appending a message and icon after creation
        let mut whisper = Whisper::new().message("Test creating a Whisper instance with message");
        assert_eq!(whisper.icon_kind, None);
        assert_eq!(
            whisper.messages,
            vec!["Test creating a Whisper instance with message"]
        );

        whisper = whisper
            .message("Append a message and icon after creation")
            .icon(IconKind::NerdFontInformation);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NerdFontInformation));
        assert_eq!(
            whisper.messages,
            vec![
                "Test creating a Whisper instance with message",
                "Append a message and icon after creation"
            ]
        );
    }

    #[test]
    fn test_whisper_default() {
        // Test default
        let whisper = Whisper::default()
            .icon(IconKind::NerdFontProcessing)
            .message("Test default");
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NerdFontProcessing));
    }
}
