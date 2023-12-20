
 # murmur
[![github](https://img.shields.io/badge/github-andretcarpizo/murmur-blue.svg)](https://github.com/andretcarpizo/murmur)
[![Crates.io](https://img.shields.io/crates/v/murmur.svg)](https://crates.io/crates/murmur)
[![Documentation](https://docs.rs/murmur/badge.svg)](https://docs.rs/murmur)


 Table of Contents
1. [Usage](#usage)
2. [`IconKind` Variants](#iconkind-variants)
3. [`Whisper` Methods](#whisper-methods)
4. [Handling Errors with Default Methods](#handling-errors-with-default-methods)
5. [Customizing Error Handling](#customizing-error-handling)
6. [Integrating thiserror](#integrating-thiserror)

This library provides a simple and flexible way to format colored stdout messages with optional `NerdFonts` or `Unicode` icons.

 ```toml
 [dependencies]
 murmur = "0.1.0"
 ```
 ## Usage

 This crate provides a `Whisper` struct and an `IconKind` enum.

 ```rust
 use murmur::{Whisper, IconKind};
 ```
 ## `IconKind` Variants

 The `IconKind` enum variants map to a specific icon, each icon has a default color.
 Casing conforms to [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html).

  * `NfFaCheck`
  * `NfFaTimes`
  * `NfFaInfoCircle`
  * `UnicodeCheckMark`
  * `UnicodeCrossMark`
  * `UnicodeWarningSign`
  ...

  For a full list of the currently supported icons, see the `IconKind` enum.

 ```rust
 use murmur::{Whisper, IconKind};
 use owo_colors::OwoColorize;

     Whisper::new()
     .icon(IconKind::NfFaCheck)  default color is green
     .message("message")  green
     .message("message".red())  override this message color to red
     .whisper()
     .unwrap();
 ```

 You must have [NerdFonts](https://www.nerdfonts.com/) installed to use the `Nf` variants.
 - [Nerfonts github](https://github.com/ryanoasis/nerd-fonts?tab=readme-ov-files)
 - [NerdFonts cheat-sheet](https://www.nerdfonts.com/cheat-sheet)

 ## `Whisper` methods:

 The `Whisper` struct is a fluent builder that provides the following methods:

 - `new()`: Creates a new `Whisper` instance
 - `.icon()`: Adds an icon to the `Whisper` instance
 - `.message()`: Adds a message to the `Whisper` instance
 - `.message_vec()`: Add a vector of messages to the `Whisper` instance
 - `.whisper()`: Builds the `Whisper` instance and prints the messages

 Here are some examples of how to use the `Whisper` struct.

 ### Creating a new `Whisper` instance, adding an icon and a message

 ```rust
 use murmur::{Whisper, IconKind};

 Whisper::new()
     .icon(IconKind::NfFaCheck)
     .message("message")
     .whisper()
     .unwrap();
 ```
 ### Adding a chain of messages to the `Whisper` instance without an icon

 ```rust
 use murmur::Whisper;
 use std::io::{Error, ErrorKind};

 fn main() -> Result<(), Error> {
 Whisper::new()
     .message("1 message without icon")
     .message("2 message all messages after the first indents by 2 spaces")
     .message("3 message")
     .whisper()
     .map_err(|err| Error::new(ErrorKind::Other, err))?;
    Ok(())
 }
 ```

 output:

 ``` text
 1 message without icon
   2 message without icon indents by 2 spaces all messages after the first
   3 message
 ```

 ### Adding a vector of messages to the `Whisper` instance

 ```rust
 use murmur::{Whisper, IconKind};
 use std::error::Error;

 fn main() -> Result<(), Box<dyn Error>> {
 let whisper = Whisper::new()
     .icon(IconKind::NfFaBug)
     .message_vec(vec["1 message without icon", "2 message", "3 message"])
     .whisper()
     .map_err(|err| err)?;
   Ok(())
 }
 ```

 ## Handling Errors with Default Methods

 The `whisper` function returns  `-> Result<(), WhisperError>`

 ```rust
 use murmur::{Whisper, IconKind, WhisperError};
 use std::io::{Error, ErrorKind};

 fn whisper_new() -> Result<(), WhisperError> {
     let whisper = Whisper::new()
         .icon(IconKind::NfFaBug)
         .message("creating a `Whisper` instance.")
         .whisper()?;
     Ok(())
 }


 fn whisper_unwrap() {
     Whisper::new()
         .icon(IconKind::NfFaInfoCircle)
         .message("unwrap")
         .message("Returns the contained Ok value, consuming the self value,function may panic, its use is generally discouraged")
         .whisper()
         .unwrap();
 }

 fn whisper_unwrap_or_else() {
     Whisper::new()
         .icon(IconKind::NfFaBug)
         .message("unwrap_or_else")
         .message("Unwrapping a `Whisper` instance or panicking with a custom message.")
         .whisper()
         .unwrap_or_else(|err| panic("Failed to print message: {}", err));
 }

 fn whisper_expect() {
     Whisper::new()
         .icon(IconKind::NfFaWarning)
         .message("expect")
         .message(
             "Returns the contained Ok value, consuming the self value.\
              Because this function may panic, its use is generally discouraged.\
              Instead, prefer to use pattern matching and handle the Err case explicitly,\
              or call unwrap_or, unwrap_or_else, or unwrap_or_default.",
         )
         .whisper()
         .expect("Failed to print message");
 }

 fn whisper_map_err() -> Result<(), Error> {
     Whisper::new()
         .icon(IconKind::NfFaTimes)
         .message("map_err")
         .message("Maps a Result<T, E> to Result<T, F> by applying a function to a contained Err value, leaving an Ok value untouched.")
         .message("This function can be used to pass through a successful result while handling an error.")
         .whisper()
         .map_err(|err| Error::new(ErrorKind::Other, err))?;
     Ok(())
 }

 fn whisper_ok() {
     Whisper::new()
         .icon(IconKind::NfFaTimes)
         .message("ok")
         .message("Converts from Result<T, E> to Option<T>.")
         .message("consuming self, and discarding the error, if any.")
         .whisper()
         .ok();
 }

 fn whisper_box_dyn_error() -> Result<(), Box<dyn std::error::Error>> {
    Whisper::new()
        .icon(IconKind::NfFaTimes)
       .message("box_dyn_error")
       .message("This function returns a Result. If the operation is successful, it returns Ok(()).")
       .message("If there is an error during the operation, it returns WhisperError.")
       .whisper()?;
    Ok(())
 }
 fn whisper_match() {
     let whisper = Whisper::new()
         .icon(IconKind::NfFaBug)
         .message("match")

     match whisper.whisper() {
         Ok(()) => println("Message printed successfully"),
         Err(error) => eprintln("Failed to print message: {error}",),
     }
 }

 fn whisper_if_let() {
     let whisper = Whisper::new()
         .icon(IconKind::NfFaBug)
         .message("if_let")
         .message("test_whisper_if_let");

     if let Err(error) = whisper.whisper() {
         eprintln("Failed to print message: {error}",);
     }
 }
 ```
 ## Customizing Error Handling
 ```rust
 use murmur::{Whisper, IconKind, WhisperError};

 #[derive(Debug)]
 enum CustomError {
     WhisperError(String),
 }

impl From<WhisperError> for CustomError {
  fn from(error: WhisperError) -> Self {
    Self::WhisperError(format!("We can add more info to the error: {error}"))
  }
}

 fn explicit_closure_for_error_conversion() -> Result<(), CustomError> {
     Whisper::new()
         .icon(IconKind::NfFaTimes)
         .message("Using an explicit closure to convert a `WhisperError` into a `CustomError`.")
         .whisper()
         .map_err(|err| CustomError::from(err))?;
     Ok(())
 }

 fn function_reference_for_error_conversion() -> Result<(), CustomError> {
     Whisper::new()
         .icon(IconKind::NfFaTimes)
         .message("Using a function reference to convert a `WhisperError` into a `CustomError`.")
         .whisper()
         .map_err(CustomError::from)?;
     Ok(())
 }
 ```
 ### Integrating thiserror
 ```rust
 use thiserror::Error;
 use murmur::{Whisper, IconKind, WhisperError};

 #[derive(Error, Debug)]
 enum CustomError {
    #[error("We can add more info to the error: {0}")]
   MyError(#[from] WhisperError),

   #[error("We can add more info to the error")]
   OtherError(),
 }

 fn thiserror_error_conversion() -> Result<(), CustomError> {
    Whisper::new()
      .icon(IconKind::NfFaTimes)
      .message("Using thiserror")
      .whisper()
      .map_err(CustomError::from)?;
   Ok(())
  }
 ```
