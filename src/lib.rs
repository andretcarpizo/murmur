//! [![GitHub](https://img.shields.io/badge/github-murmur-blue.svg)](https://github.com/andretcarpizo/murmur)
//! [![Crates.io](https://img.shields.io/crates/v/murmur.svg)](https://crates.io/crates/murmur)
//! [![Documentation](https://docs.rs/murmur/badge.svg)](https://docs.rs/murmur)
//! [![GitHub Actions](https://github.com/andretcarpizo/murmur/actions/workflows/rust.yml/badge.svg)](https://github.com/andretcarpizo/murmur/actions)
//! [![License](https://img.shields.io/crates/l/murmur.svg)](https://github.com/andretcarpizo/murmur/blob/main/LICENSE.md)
//!
//! A flexible library to build messages with  `NerdFonts` or `Unicode` icons.
//!
//! Table of Contents
//! 1. [Introduction](#Introduction)
//! 2. [`IconKind` Variants](#iconkind-variants)
//! 3. [`Whisper` Methods](#whisper-methods)
//!    - [`new()`](#new)
//!    - [`icon()`](#icon)
//!    - [`message()`](#message)
//!    - [`messages()`](#messages)
//!    - [`whisper()`](#whisper)
//! 4. [`WhisperError`](#whispererror)
//! 5. [Examples](https://github.com/andretcarpizo/murmur/tree/main/examples)
//!
//! ## Introduction
//!
//! There is only a `Whisper` struct and an `IconKind` enum.
//!
//! ```rust
//! use murmur::{Whisper, IconKind};
//! ```
//!
//! ## `IconKind` Variants
//!
//! The `IconKind` enum variants map to a specific `Unicode` or `NerdFont` icon, each icon has a default color.
//! Casing conforms to [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html).
//!
//! - `NfFaTimes`
//! - `NfFaCheck`
//! - `NfFaInfoCircle`
//! - `NfFaRefresh`
//! - `NfFaWarning`
//! - `NfFaBug`
//! - `UnicodeCrossMark`
//! - `UnicodeCheckMark`
//! - `UnicodeInformationSource`
//! - `UnicodeGear`
//! - `UnicodeWarningSign`
//! - `UnicodeBug`
//!
//!
//!  For a full list of the currently supported icons, see the `IconKind` [enum](https://docs.rs/murmur/1.2.1/murmur/enum.IconKind.htmlhttps://docs.rs/murmur/1.2.1/murmur/enum.IconKind.html).
//! ```rust
//! use murmur::{Whisper, IconKind};
//! use owo_colors::OwoColorize;
//!
//! Whisper::new()
//!     .icon(IconKind::NfFaCheck)
//!     .message("message")
//!     .message("message".red())
//!     .whisper()
//!     .unwrap();
//!
//!
//! ```
//!
//! <div class="warning">You must have NerdFonts installed to use the `Nf` variants.</div>
//!
//! - [NerdFonts GitHub](https://github.com/ryanoasis/nerd-fonts?tab=readme-ov-files)
//! - [NerdFonts cheat-sheet](https://www.nerdfonts.com/cheat-sheet)
//!
//! ## `Whisper` methods:
//!
//! The `Whisper` struct provides the following methods:
//!
//! - `new()`: Creates a new `Whisper` instance
//! - `.icon()`: Adds an icon to the `Whisper` instance
//! - `.message()`: Adds a message to the `Whisper` instance
//! - `.messages()`: Adds multiple messages to the `Whisper` instance
//! - `.whisper()`: Builds the `Whisper` instance and prints the messages
//!
//! Here are some examples of how to use the `Whisper` struct.
//!
//! ### new
//!
//! ```rust
//! use murmur::{Whisper, IconKind};
//!
//! Whisper::new()
//!     .icon(IconKind::NfFaCheck)
//!     .message("message")
//!     .whisper()
//!     .ok();
//! ```
//!
//! ### icon
//! ```rust
//! use murmur::{Whisper, IconKind};
//!
//! Whisper::new().icon(IconKind::UnicodeCheckMark).whisper().ok();
//! ```
//!
//! ### message
//!
//! ```rust
//! use murmur::Whisper;
//! use std::io::{Error, ErrorKind};
//!
//! fn main() -> Result<(), Error> {
//!     Whisper::new()
//!         .message("1 message")
//!         .message("2 message")
//!         .message("3 message")
//!         .whisper()
//!         .map_err(|err| Error::new(ErrorKind::Other, err))?;
//!     Ok(())
//! }
//! ```
//! Output:
//!
//! ```text
//! 1 message without icon
//!   2 message without icon indents by 2 spaces all messages after the first
//!   3 message
//! ```
//!
//! ### messages
//!
//! ```rust
//! use murmur::Whisper;
//!
//! Whisper::new()
//!     .messages(["1 message without icon", "2 message", "3 message"])
//!     .whisper()
//!     .ok();
//!
//! Whisper::new()
//!     .messages(vec!["1 message without icon", "2 message", "3 message"])
//!     .whisper()
//!     .ok();
//! ```
//!`WhisperError`
//!
//! The `whisper` method returns  `-> Result<(), WhisperError>`
//!
//! ```rust
//! use murmur::{Whisper, IconKind, WhisperError};
//! use std::io::{Error, ErrorKind};
//!
//!
//!
//! fn whisper_new() -> Result<(), WhisperError> {
//!     let whisper = Whisper::new()
//!         .icon(IconKind::NfFaBug)
//!         .message("The `whisper` method returns  `-> Result<(), WhisperError>`")
//!         .whisper()?;
//!     Ok(())
//! }
//!```

#![doc(html_root_url = "https://docs.rs/murmur/")]
mod color_map;
mod icon_map;

// Re-exports
pub use icon_map::IconKind;

use core::fmt::{Debug, Display};
use std::fmt;
use std::io::{self, BufWriter, Write};

/// The `WhisperError` enum represents different kinds of errors that can occur while printing messages.
#[derive(Debug)]
pub enum WhisperError {
    /// Error acquiring lock on ICON_MAP
    Lock,

    /// Error printing message
    Print,

    /// Error writing to buffer
    Write,

    /// Error flushing buffer
    Flush,

    /// Error converting bytes to UTF-8 string
    Utf8Conversion,
}

impl Display for WhisperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lock => write!(f, "Failed to acquire lock on ICON_MAP"),
            Self::Print => write!(f, "Failed to print message"),
            Self::Write => write!(f, "Error writing to buffer"),
            Self::Flush => write!(f, "Error flushing buffer"),
            Self::Utf8Conversion => write!(f, "Failed to convert bytes to UTF-8 string"),
        }
    }
}

impl std::error::Error for WhisperError {}

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
/// use murmur::{Whisper, IconKind};
///
/// let whisper = Whisper::new()
///     .icon(IconKind::NfFaBug)
///     .message("test_whisper_unwrap")
///     .whisper()
///     .ok();
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Whisper {
    /// An optional field that specifies the kind of icon to be displayed.
    pub icon_kind: Option<IconKind>,
    /// A vector of messages to be displayed.
    pub messages: Vec<String>,
}

impl Whisper {
    /// Creates a new `Whisper` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use murmur::{Whisper, IconKind};
    ///
    /// let whisper = Whisper::new()
    ///     .icon(IconKind::NfFaBug)
    ///     .message("message")
    ///     .whisper()
    ///     .ok();
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
    /// * `icon_kind`: The kind of icon to be displayed. See the `IconKind` enum for a list of available icons.
    ///
    /// # Returns
    ///
    /// A `Whisper` instance with the specified icon.
    ///
    /// # Examples
    ///
    /// ```
    /// use murmur::{Whisper, IconKind};
    /// let whisper = Whisper::new()
    ///     .icon(IconKind::NfFaCheck)
    ///     .message("Nerd Font icons if you have them installed")
    ///     .message("Another message")
    ///     .whisper()
    ///     .expect("Failed to print message");
    ///
    /// let whisper = Whisper::new()
    ///     .icon(IconKind::UnicodeWarningSign)
    ///     .message("Unicode icons")
    ///     .whisper()
    ///     .or_else(|err| Err(err));
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
    /// use murmur::Whisper;
    /// let whisper = Whisper::new()
    ///    .message("1 message without icon")
    ///    .message("2 message")
    ///    .message("3 message")
    ///    .whisper()
    ///    .ok();
    /// ```
    /// # Output
    /// ```text
    /// 1 message without icon
    ///   2 message without icon indents by 2 spaces all messages after the first
    ///   3 message
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
    /// * `messages`: An iterable collection of items, each implementing `Display`, `Debug`, and `AsRef<str>`.
    ///
    /// Returns a `Whisper` instance with the added messages.
    ///
    /// # Examples
    ///
    /// ```
    /// use murmur::Whisper;
    ///
    /// Whisper::new()
    ///     .messages(["1 message", "2 message", "3 message"])
    ///     .whisper()
    ///     .ok();
    ///
    /// Whisper::new()
    ///     .messages(vec!["1 message", "2 message", "3 message"])
    ///     .whisper()
    ///     .ok();
    ///
    /// ```
    /// # Output
    /// ```text
    /// 1 message
    ///   2 message
    ///   3 message
    /// ```
    #[must_use]
    pub fn messages<I, S>(mut self, messages: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Display + Debug + AsRef<str>,
    {
        for message in messages {
            self.messages.push(message.as_ref().to_string());
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
    /// This function returns a `Result`. If the operation is successful, it returns `Ok(())`. If there is an error during the operation, it returns `WhisperError`.
    ///
    /// # Errors
    ///
    /// This function will return `WhisperError::Lock` if it fails to acquire a lock on the `ICON_MAP`.
    /// It will return `WhisperError::Print` if there is an error while printing the messages.
    ///
    /// # Example
    ///
    /// ```rust
    /// use murmur::{Whisper, IconKind};
    /// use std::io::{Error, ErrorKind};
    ///
    /// fn main() -> Result<(), Error> {
    ///     let whisper = Whisper::new()
    ///         .icon(IconKind::NfFaBug)
    ///        .message("test_whisper_unwrap")
    ///        .whisper()
    ///        .map_err(|err| Error::new(ErrorKind::Other, err))?;
    ///     Ok(())
    /// }
    /// ```
    pub fn whisper(&self) -> Result<(), WhisperError> {
        // Try to lock the ICON_MAP for safe access in a concurrent environment
        let icon_map = icon_map::ICON_MAP.lock().map_err(|_| WhisperError::Lock)?;

        // Check the icon_kind field of the Whisper instance
        let (icon, color) = self.icon_kind.clone().map_or(("", ""), |icon_kind| {
            icon_map.get(&icon_kind).map_or(("", ""), |value| *value)
        });

        // Print the messages with the specified color and an optional icon prefix
        self.print_messages(icon, color)
            .map_err(|_| WhisperError::Print)?;

        Ok(())
    }

    /// Writes the output of a process as a whisper.
    ///
    /// This function is only available when the `experimental` feature is enabled.
    /// It takes a reference to the output of a process and converts it to a UTF-8 string.
    /// Then, it creates a `Whisper` instance with the extracted message and invokes the
    /// `whisper` method to perform the whispering process.
    ///
    /// # Arguments
    ///
    /// * `output` - The output of a process as a `std::process::Output` reference.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the whispering process is successful, otherwise returns a `WhisperError`.
    ///
    /// # Errors
    ///
    /// The function may return a `WhisperError` if:
    ///
    /// * The conversion from the output to a UTF-8 string fails.
    /// * The whispering process fails.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    #[cfg(feature = "experimental")]
    pub fn whisper_out(self, output: &std::process::Output) -> Result<(), WhisperError> {
        let message =
            std::str::from_utf8(&output.stdout).map_err(|_| WhisperError::Utf8Conversion)?;
        let whisper = self.message(message);
        whisper.whisper()?;
        Ok(())
    }

    /// This function is used to handle error messages from a process output and send them as whispers.
    /// It is only available when the "experimental" feature is enabled.
    ///
    /// # Arguments
    ///
    /// * `output` - Process output that contains the error message.
    ///
    /// # Returns
    ///
    /// This function returns a `Result<(), WhisperError>`. If the conversion from `stderr` to a string fails,
    /// an `Utf8Conversion` variant of `WhisperError` is returned. Otherwise, the message is sent as a whisper
    /// and the function returns `Ok()`.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// ```
    ///
    /// # Errors
    ///
    #[cfg(feature = "experimental")]
    pub fn whisper_err(self, output: &std::process::Output) -> Result<(), WhisperError> {
        let message =
            std::str::from_utf8(&output.stderr).map_err(|_| WhisperError::Utf8Conversion)?;
        let whisper = self.message(message);
        whisper.whisper()?;
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
    /// If there is an error while printing the messages, it returns `WhisperError::Print`.
    ///
    /// # Arguments
    ///
    /// * `icon`: A string slice that represents the icon to be printed before each message.
    /// * `color`: A string slice that represents the color of the messages and the icon.
    ///
    /// # Returns
    ///
    /// This function returns a `Result`. If the operation is successful, it returns `Ok(())`. If there is an error during the operation, it returns `WhisperError`.
    ///
    /// # Errors
    ///
    /// This function will return `WhisperError::Print` if there is an error while printing the messages.
    fn print_messages(&self, icon: &str, color: &str) -> Result<(), WhisperError> {
        let messages = if self.messages.is_empty() {
            vec![String::new()]
        } else {
            self.messages.clone()
        };

        for (index, message) in messages.iter().enumerate() {
            let prefix = if index == 0 { icon } else { "  " };
            Self::print_message(color, prefix, message).map_err(|_| WhisperError::Print)?;
        }
        Ok(())
    }

    /// Prints a message to stdout with a specific color and prefix.
    ///
    /// This function is responsible for printing a message to stdout with a specific color and prefix.
    /// It first creates a buffer writer with a specific buffer size for stdout.
    /// Then, it checks if the color exists in the `COLOR_MAP`. If it does, it locks the `Mutex` to get a mutable reference to the function.
    /// After that, it calls the function with the prefix and message as arguments and writes the result to the buffer writer.
    /// If the color does not exist in the `COLOR_MAP`, it writes the prefix and message directly to the buffer writer.
    /// Finally, it flushes the buffer writer to ensure that all data is written to stdout.
    ///
    /// # Arguments
    ///
    /// * `color`: A string slice that represents the color of the message.
    /// * `prefix`: A string slice that represents the prefix to be printed before the message.
    /// * `message`: A string slice that represents the message to be printed.
    ///
    /// # Returns
    ///
    /// This function returns a `Result`. If the operation is successful, it returns `Ok(())`. If there is an error during the operation, it returns `WhisperError`.
    ///
    /// # Errors
    ///
    /// This function will return `WhisperError::Lock` if it fails to acquire a lock on the `Mutex`.
    /// It will return `WhisperError::Write` if there is an error while writing to the buffer.
    /// It will return `WhisperError::Flush` if there is an error while flushing the buffer.
    fn print_message(color: &str, prefix: &str, message: &str) -> Result<(), WhisperError> {
        /// The buffer size for stdout, 8192 bytes.
        const BUFFER_SIZE: usize = 8192;
        let stdout = io::stdout();
        let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout.lock());

        if let Some(color_fn) = color_map::COLOR_MAP.get(color) {
            writeln!(writer, "{}{}", color_fn(prefix), color_fn(message))
                .map_err(|_| WhisperError::Write)?;
        } else {
            writeln!(writer, "{prefix}{message}").map_err(|_| WhisperError::Write)?;
        }

        writer.flush().map_err(|_| WhisperError::Flush)?;
        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "experimental")]
mod whisper_experimental {
    use super::*;
    use std::error::Error;
    use std::process::Command;

    //cargo test --features experimental --color=always --package murmur --lib whisper_experimental
    #[test]
    fn execute_cargo_version() -> Result<(), Box<dyn Error>> {
        // Declaration and initialization of whisper
        let success = Whisper::new().icon(IconKind::NfFaCheck);
        let failure = Whisper::new().icon(IconKind::NfFaTimes);

        // Usage of whisper in application logic
        let output = Command::new("cargo").arg("version").output()?;

        if output.status.success() {
            success.whisper_out(&output)?;
        } else {
            failure.whisper_err(&output)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod whisper_color_override_tests {
    use super::*;
    use owo_colors::OwoColorize;

    #[test]
    fn test_whisper_color_print() {
        Whisper::new()
            .message("test color".yellow())
            .whisper()
            .unwrap();
    }

    #[test]
    fn test_whisper_with_icon_and_color() {
        Whisper::new()
            .icon(IconKind::NfFaCheck)
            .message("each icon provides a default color but you can override it")
            .message("this message is red".red())
            .whisper()
            .unwrap();
    }

    #[test]
    fn test_add_custom_color() {
        Whisper::new()
            .icon(IconKind::NfFaBug)
            .message("test color".magenta())
            .message("owo_colors crate has a lot of colors".cyan())
            .message("you can add your own colors".red())
            .whisper()
            .unwrap();
    }
}

#[cfg(test)]
mod whisper_functionality_tests {
    use super::*;

    #[test]
    fn test_whisper_messages() {
        Whisper::new()
            .messages(["1 message without icon", "2 message", "3 message"])
            .whisper()
            .ok();

        Whisper::new()
            .messages(vec!["1 message without icon", "2 message", "3 message"])
            .whisper()
            .ok();

        Whisper::new()
            .messages(["1 message without icon", "2 message", "3 message"].iter())
            .whisper()
            .ok();
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
        let whisper = Whisper::new().icon(IconKind::NfFaBug);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NfFaBug));
        assert_eq!(whisper.messages, Vec::<String>::new());
    }

    #[test]
    fn test_whisper_icon_message() {
        // After
        let whisper = Whisper::new()
            .icon(IconKind::NfFaInfoCircle)
            .message("message with icon");
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NfFaInfoCircle));
        assert_eq!(whisper.messages.as_slice(), &["message with icon"]);
    }

    #[test]
    fn test_whisper_icon_multiple_messages() {
        // After
        let whisper = Whisper::new()
            .icon(IconKind::NfFaWarning)
            .message("First message")
            .message("Second message")
            .message("Third message");
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NfFaWarning));
        assert_eq!(
            whisper.messages.as_slice(),
            &["First message", "Second message", "Third message"]
        );
    }

    #[test]
    fn test_whisper_icon_message_and_multiple_messages() {
        // After
        let whisper = Whisper::new()
            .icon(IconKind::NfFaCheck)
            .message("First message")
            .message("Second message")
            .messages(vec!["Third message", "Fourth message"]);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NfFaCheck));
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
    fn test_icon_multiple_message_and_messages() {
        let whisper = Whisper::new()
            .icon(IconKind::NfFaWarning)
            .messages(vec!["Line", "Another line"])
            .messages(vec!["Another line"]);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(
            whisper.messages,
            vec!["Line", "Another line", "Another line"]
        );
    }
    #[test]
    fn test_message_empty_messages() {
        // Test for the `message_vec` method when the `messages` vector is empty.
        let whisper = Whisper::new().messages(Vec::<String>::new());
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, None);
        assert_eq!(whisper.messages, Vec::<String>::new());
    }

    #[test]
    fn test_message_multiple_messages() {
        // Test for the `message_vec` method when the `messages` vector contains multiple elements.
        let whisper = Whisper::new()
            .icon(IconKind::NfFaTimes)
            .messages(vec!["Test message vec 1", "Test message vec 2"]);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NfFaTimes));
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
            .icon(IconKind::NfFaBug)
            .message("icon should be added to the first message");
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NfFaBug));
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
            .icon(IconKind::NfFaInfoCircle);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NfFaInfoCircle));
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
            .icon(IconKind::NfFaRefresh)
            .message("Test default");
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.icon_kind, Some(IconKind::NfFaRefresh));
    }

    #[test]
    fn test_whisper_very_long_message() {
        let long_message = "a".repeat(10000);
        let whisper = Whisper::new()
            .icon(IconKind::NfFaBug)
            .message(long_message.clone());
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.messages, vec![long_message]);
    }

    #[test]
    fn test_whisper_special_characters_in_message() {
        let special_message = "!@#$%^&*()";
        let whisper = Whisper::new()
            .icon(IconKind::NfFaBug)
            .message(special_message);
        let result = whisper.whisper();
        assert!(result.is_ok());
        assert_eq!(whisper.messages, vec![special_message]);
    }

    #[test]
    fn test_whisper_large_number_of_messages() {
        let messages = vec!["Test message".to_string(); 1000];
        let whisper = Whisper::new()
            .icon(IconKind::NfFaBug)
            .messages(messages.clone());
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
        let error = WhisperError::Lock;
        assert_eq!(format!("{error}"), "Failed to acquire lock on ICON_MAP");
    }

    #[test]
    fn whisper_error_print_error() {
        let error = WhisperError::Print;
        assert_eq!(format!("{error}"), "Failed to print message");
    }

    #[test]
    fn whisper_error_write_error() {
        let error = WhisperError::Write;
        assert_eq!(format!("{error}"), "Error writing to buffer");
    }

    #[test]
    fn whisper_error_flush_error() {
        let error = WhisperError::Flush;
        assert_eq!(format!("{error}"), "Error flushing buffer");
    }
}
