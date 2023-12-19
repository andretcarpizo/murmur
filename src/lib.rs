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

use core::fmt::{Debug, Display};
use std::io::{self, BufWriter, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WhisperError {
    #[error("Failed to acquire lock on ICON_MAP")]
    LockError,

    #[error("Failed to print message")]
    PrintError,

    #[error("Error writing to buffer")]
    WriteError,

    #[error("Error flushing buffer")]
    FlushError,
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
    /// ```
    #[must_use]
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
    /// ```
    /// ```
    #[must_use]
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
    /// ```
    #[must_use]
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
    /// ```
    /// ```
    pub fn whisper(&self) -> Result<(), WhisperError> {
        // Try to lock the ICON_MAP for safe access in a concurrent environment
        let icon_map = icon_map::ICON_MAP
            .lock()
            .map_err(|_| WhisperError::LockError)?;

        // Check the icon_kind field of the Whisper instance
        let (icon, color) = self.icon_kind.clone().map_or(("", ""), |icon_kind| {
            icon_map.get(&icon_kind).map_or(("", ""), |value| *value)
        });

        // Print the messages with the specified color and an optional icon prefix
        self.print_messages(icon, color)
            .map_err(|_| WhisperError::PrintError)?;

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
    fn print_messages(&self, icon: &str, color: &str) -> Result<(), WhisperError> {
        let messages = if self.messages.is_empty() {
            vec![String::new()]
        } else {
            self.messages.clone()
        };

        for (index, message) in messages.iter().enumerate() {
            let prefix = if index == 0 { icon } else { "  " };
            Self::print_message(color, prefix, message)
                .map_err(|_| WhisperError::PrintError)?;
        }
        Ok(())
    }

    /// Prints a message with a specific color and an optional icon prefix.
    ///
    /// This function takes in a reference to a `ColorMap`, a color, a prefix, and a message as parameters.
    /// The `ColorMap` is used to map color names to functions that colorize a string.
    /// The color parameter is a string slice that represents the color of the message and the prefix.
    /// The prefix parameter is a string slice that represents the icon to be printed before the message.
    /// The message parameter is a string slice that represents the message to be printed.
    ///
    /// The function first creates a `BufWriter` with a buffer size of 8192 to write to stdout.
    /// It then checks if the color exists in the `ColorMap`. If it does, it uses the color function to colorize the prefix and the message.
    /// If the color does not exist in the `ColorMap`, it prints the prefix and the message as is.
    ///
    /// After printing the message, it flushes the `BufWriter` to ensure all data is written to stdout.
    ///
    /// # Arguments
    ///
    /// * `color_map`: A reference to a `ColorMap` that maps color names to functions that colorize a string.
    /// * `color`: A string slice that represents the color of the message and the prefix.
    /// * `prefix`: A string slice that represents the icon to be printed before the message.
    /// * `message`: A string slice that represents the message to be printed.
    ///
    /// # Returns
    ///
    /// This function returns a `Result`. If the operation is successful, it returns `Ok(())`. If there is an error during the operation, it returns `Err(WhisperError)`.
    ///
    /// # Errors
    ///
    /// This function will return `Err(WhisperError::WriteError)` if there is an error while writing to the `BufWriter`.
    /// It will return `Err(WhisperError::FlushError)` if there is an error while flushing the `BufWriter`.
    pub fn print_message(
        color: &str,
        prefix: &str,
        message: &str,
    ) -> Result<(), WhisperError> {
        /// The capacity of the buffer used to write to stdout.
        const BUFFER_SIZE: usize = 8192;
        let stdout = io::stdout();
        let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout.lock());

        if let Some(color_fn) = color_map::COLOR_MAP.get(color) {
            writeln!(writer, "{}{}", color_fn(prefix), color_fn(message))
                .map_err(|_| WhisperError::WriteError)?;
        } else {
            writeln!(writer, "{prefix}{message}").map_err(|_| WhisperError::WriteError)?;
        }

        writer.flush().map_err(|_| WhisperError::FlushError)?;
        Ok(())
    }
}

#[cfg(test)]
mod whisper_tests {
    use super::*;
    use std::io::{Error, ErrorKind};
    /// Test for propagating a `WhisperError`.
    ///
    /// This test creates a new `Whisper` instance, sets the icon and message,
    /// and calls the `whisper` method.
    /// If an error occurs, it is propagated up the call stack, the error is not converted. from its original type.
    #[test]
    fn test_whisper_propagate_to_murmur_error() -> Result<(), WhisperError> {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_propagate_to_murmur_error")
            .whisper()?;
        Ok(())
    }

    /// Test for unwrapping a `Whisper` instance.
    ///
    /// This test creates a new `Whisper` instance, sets the icon and message,
    /// and calls the `whisper` method.
    /// If an error occurs, the test will panic.
    #[test]
    fn test_whisper_unwrap() {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_unwrap")
            .whisper()
            .unwrap();
    }

    /// Test for unwrapping a `Whisper` instance or panicking with a custom message.
    ///
    /// This test creates a new `Whisper` instance, sets the icon and message,
    /// and calls the `whisper` method.
    /// If an error occurs, the test will panic with a custom message.
    #[test]
    fn test_whisper_unwrap_or_else() {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_unwrap_or_else")
            .whisper()
            .unwrap_or_else(|err| panic!("Failed to print message: {err}"));
    }

    /// Test for expecting a `Whisper` instance to be `Ok`.
    ///
    /// This test creates a new `Whisper` instance, sets the icon and message,
    /// and calls the `whisper` method.
    /// If an error occurs, the test will panic with a custom message.
    #[test]
    fn test_whisper_expect() {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_unwrap")
            .whisper()
            .expect("Failed to print message");
    }

    /// Test for propagating a `WhisperError` as an `io::Error`.
    ///
    /// This test creates a new `Whisper` instance, sets the icon and message,
    /// and calls the `whisper` method.
    /// If a `WhisperError` occurs, it is converted to an `io::Error` and propagated up the call stack.
    /// The test will pass if no error occurs.
    #[test]
    fn test_whisper_map_err_and_propagate_io_error() -> Result<(), Error> {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_user_api_map_err_and_propagate_io_error")
            .whisper()
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        Ok(())
    }

    /// Test for propagating a `WhisperError` as a `CustomError`.
    ///
    /// This test creates a new `Whisper` instance, sets the icon and message,
    /// and calls the `whisper` method. If a `WhisperError` occurs, it is converted to a `CustomError` and propagated up the call stack.
    /// The test will pass if no error occurs.
    #[test]
    fn test_whisper_map_err_and_propagate_to_custom_error() -> Result<(), CustomError> {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_user_api_map_err_and_propagate_to_custom_error")
            .whisper()
            .map_err(|err| CustomError::from(err))?;
        Ok(())
    }

    /// Test for discarding a `WhisperError` if any occurs.
    ///
    /// This test creates a new `Whisper` instance, sets the icon and message,
    /// and calls the `whisper` method. If a `WhisperError` occurs, it is discarded and the result is converted to an `Option`.
    /// The test will pass if no error occurs.
    #[test]
    fn test_whisper_ok_discard_error_if_any () {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_ok_discard_error_if_any")
            .whisper()
            .ok();
    }

    /// Test for handling a `WhisperError` using `or_else`.
    ///
    /// This test creates a new `Whisper` instance, sets the icon and message,
    /// and calls the `whisper` method. If a `WhisperError` occurs, it is handled by the `or_else` method which returns the error.
    /// The test will pass if no error occurs.
    #[test]
    fn test_whisper_or_else () -> Result<(), WhisperError> {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_or_else")
            .whisper()
            .or_else(|err| Err(err))
    }

    #[derive(Debug)]
    enum CustomError {
        WhisperError(String),
        // Add other kinds of errors here
    }

    impl From<WhisperError> for CustomError {
        fn from(error: WhisperError) -> Self {
            CustomError::WhisperError(format!("We can add more info to the error: {:?}", error))
        }
    }

    /// Test for propagating a `WhisperError` as a `CustomError`.
    ///
    /// This test creates a new `Whisper` instance, sets the icon and message,
    /// and calls the `whisper` method.
    /// If a `WhisperError` occurs, it is converted to a `CustomError` with the original error and propagated up the call stack.
    #[test]
    fn test_whisper_map_err_and_propagate_to_original_error_and_custom_error(
    ) -> Result<(), CustomError> {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_map_err_and_propagate_to_original_error_and_custom_error")
            .whisper()
            .map_err(|err| CustomError::from(err))?; // Ok or Convert from WhisperError to CustomError, pass the original error and Propagate Error

        Ok(())
    }

    /// If a `WhisperError` occurs, it is converted to a `CustomError` the original error is not propagated up the call stack.
    #[test]
    fn test_whisper_map_err_and_convert_to_custom_error_discard_original_error(
    ) -> Result<(), CustomError> {
        Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_map_err_and_convert_to_custom_error_discard_original_error")
            .whisper()
            .map_err(CustomError::from)?; // Ok or Convert from WhisperError to CustomError and Propagate Error, dont pass the original
        Ok(())
    }

    #[test]
    fn test_whisper_match() {
        let whisper = Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message("test_whisper_match");

        match whisper.whisper() {
            Ok(()) => println!("Message printed successfully"),
            Err(error) => eprintln!("Failed to print message: {error}",),
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

    #[test]
    fn test_whisper_very_long_message() {
        let long_message = "a".repeat(10000);
        let whisper = Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message(long_message.clone());
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.messages, vec![long_message]);
    }

    #[test]
    fn test_whisper_special_characters_in_message() {
        let special_message = "!@#$%^&*()";
        let whisper = Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message(special_message);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.messages, vec![special_message]);
    }

    #[test]
    fn test_whisper_large_number_of_messages() {
        let messages = vec!["Test message".to_string(); 10000];
        let whisper = Whisper::new()
            .icon(IconKind::NerdFontDebugging)
            .message_vec(messages.clone());
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.messages, messages);
    }

}


#[cfg(test)]
mod whisper_error_tests {
    use super::*;

    #[test]
    fn whisper_error_lock_error() {
        let error = WhisperError::LockError;
        assert_eq!(format!("{}", error), "Failed to acquire lock on ICON_MAP");
    }

    #[test]
    fn whisper_error_print_error() {
        let error = WhisperError::PrintError;
        assert_eq!(format!("{}", error), "Failed to print message");
    }

    #[test]
    fn whisper_error_write_error() {
        let error = WhisperError::WriteError;
        assert_eq!(format!("{}", error), "Error writing to buffer");
    }

    #[test]
    fn whisper_error_flush_error() {
        let error = WhisperError::FlushError;
        assert_eq!(format!("{}", error), "Error flushing buffer");
    }
}