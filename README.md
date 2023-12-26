 # murmur

 [![GitHub](https://img.shields.io/badge/github-murmur-blue.svg)](https://github.com/andretcarpizo/murmur)
 [![Crates.io](https://img.shields.io/crates/v/murmur.svg)](https://crates.io/crates/murmur)
 [![Documentation](https://docs.rs/murmur/badge.svg)](https://docs.rs/murmur)
 [![GitHub Actions](https://github.com/andretcarpizo/murmur/actions/workflows/rust.yml/badge.svg)](https://github.com/andretcarpizo/murmur/actions)
 [![License](https://img.shields.io/crates/l/murmur.svg)](https://github.com/andretcarpizo/murmur/blob/main/LICENSE.md)

 A flexible library to build messages with  `NerdFonts` or `Unicode` icons.

 Table of Contents
 1. [Intro](#Intro)
 2. [`IconKind` Variants](#iconkind-variants)
 3. [`Whisper` Methods](#whisper-methods)
    - [`new()`](#new)
    - [`icon()`](#icon)
    - [`message()`](#message)
    - [`messages()`](#messages)
    - [`whisper()`](#whisper)
 4. [`WhisperError`](#whisper-error)
 5. [Examples](https://github.com/andretcarpizo/murmur/tree/main/examples)

 ## Intro

 There is only a `Whisper` struct and an `IconKind` enum.

 ```rust
 use murmur::{Whisper, IconKind};
 ```

 ## `IconKind` Variants

 The `IconKind` enum variants map to a specific `Unicode` or `NerdFont` icon, each icon has a default color.
 Casing conforms to [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html).

 - `NfFaTimes`
 - `NfFaCheck`
 - `NfFaInfoCircle`
 - `NfFaRefresh`
 - `NfFaWarning`
 - `NfFaBug`
 - `UnicodeCrossMark`
 - `UnicodeCheckMark`
 - `UnicodeInformationSource`
 - `UnicodeGear`
 - `UnicodeWarningSign`
 - `UnicodeBug`


  For a full list of the currently supported icons, see the `IconKind` [enum](https://docs.rs/murmur/1.2.1/murmur/enum.IconKind.htmlhttps://docs.rs/murmur/1.2.1/murmur/enum.IconKind.html).
 ```rust
 use murmur::{Whisper, IconKind};
 use owo_colors::OwoColorize;

 Whisper::new()
     .icon(IconKind::NfFaCheck)
     .message("message")
     .message("message".red())
     .whisper()
     .unwrap();


 ```

 <div class="warning">You must have NerdFonts installed to use the `Nf` variants.</div>

 - [NerdFonts GitHub](https://github.com/ryanoasis/nerd-fonts?tab=readme-ov-files)
 - [NerdFonts cheat-sheet](https://www.nerdfonts.com/cheat-sheet)

 ## `Whisper` methods:

 The `Whisper` struct provides the following methods:

 - `new()`: Creates a new `Whisper` instance
 - `.icon()`: Adds an icon to the `Whisper` instance
 - `.message()`: Adds a message to the `Whisper` instance
 - `.messages()`: Adds multiple messages to the `Whisper` instance
 - `.whisper()`: Builds the `Whisper` instance and prints the messages

 Here are some examples of how to use the `Whisper` struct.

 ### new

 ```rust
 use murmur::{Whisper, IconKind};

 Whisper::new()
     .icon(IconKind::NfFaCheck)
     .message("message")
     .whisper()
     .ok();
 ```

 ### icon
 ```rust
 use murmur::{Whisper, IconKind};

 Whisper::new().icon(IconKind::UnicodeCheckMark).whisper().ok();
 ```

 ### message

 ```rust
 use murmur::Whisper;
 use std::io::{Error, ErrorKind};

 fn main() -> Result<(), Error> {
     Whisper::new()
         .message("1 message")
         .message("2 message")
         .message("3 message")
         .whisper()
         .map_err(|err| Error::new(ErrorKind::Other, err))?;
     Ok(())
 }
 ```
 Output:

 ```text
 1 message without icon
   2 message without icon indents by 2 spaces all messages after the first
   3 message
 ```

 ### messages

 ```rust
 use murmur::Whisper;

 Whisper::new()
     .messages(["1 message without icon", "2 message", "3 message"])
     .whisper()
     .ok();

 Whisper::new()
     .messages(vec!["1 message without icon", "2 message", "3 message"])
     .whisper()
     .ok();
 ```
 ### Whisper Error

 The `whisper` method returns  `-> Result<(), WhisperError>`

 ```rust
 use murmur::{Whisper, IconKind, WhisperError};
 use std::io::{Error, ErrorKind};

 fn whisper_new() -> Result<(), WhisperError> {
     let whisper = Whisper::new()
         .icon(IconKind::NfFaBug)
         .message("The `whisper` method returns  `-> Result<(), WhisperError>`")
         .whisper()?;
     Ok(())
 }
```
