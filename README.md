
# murmur

This Rust crate provides a simple and flexible way to format colored stdout messages with optional `NerdFonts` or `Unicode` icons.

[![Crates.io](https://img.shields.io/crates/v/murmur.svg)](https://crates.io/crates/murmur)
[![Documentation](https://docs.rs/murmur/badge.svg)](https://docs.rs/murmur)
[![GitHub](https://img.shields.io/github/stars/andretcarpizo/murmur.svg?style=social)](https://github.com/andretcarpizo/murmur)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
murmur = "0.1.0"
```

This crate provides a `Whisper` struct and an `IconKind` enum.

```rust
use murmur::{Whisper, IconKind};
```
## `IconKind` Variants

The `IconKind` enum variants map to a specific icon, each icon has a default color.
Casing conforms to [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-idiomatic-rust-style).

* `NfFaCheck`
* `NfFaTimes`
* `NfFaInfoCircle`
* `UnicodeCheckMark`
* `UnicodeCrossMark`
* `UnicodeWarningSign`
  ...

For a full list of the currently supported icons, see the `IconKind` enum.

# Examples
```rust
use murmur::{Whisper, IconKind};
use owo_colors::OwoColorize;

    Whisper::new()
    .icon(IconKind::NfFaCheck) // default color is green
    .message("message") // green
    .message("message".red()) // override this message color to red
    .whisper()
    .unwrap();
```


[NerdFonts cheat-sheet](https://www.nerdfonts.com/cheat-sheet)

## `Whisper` methods:

The `Whisper` struct is a fluent builder that provides the following methods:

- `new()`: Creates a new `Whisper` instance
- `.icon()`: Adds an icon to the `Whisper` instance
- `.message()`: Adds a message to the `Whisper` instance
- `.message_vec()`: Add a vector of messages to the `Whisper` instance
- `.whisper()`: Builds the `Whisper` instance and prints the messages

## Examples

Here are some examples of how to use the `Whisper` struct.

### Creating a new `Whisper` instance adding an icon and a message

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

This will output:

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
    .message_vec(vec!["1 message without icon", "2 message", "3 message"])
    .whisper()
    .map_err(|err| err)?;
  Ok(())
}
```

## Error Handling Examples

The `whisper` function returns  `-> Result<(), WhisperError>`

```rust
use murmur::{Whisper, IconKind, WhisperError};
use std::io::{Error, ErrorKind};

fn create_whisper() -> Result<(), WhisperError> {
    let whisper = Whisper::new()
        .icon(IconKind::NfFaBug)
        .message("creating a `Whisper` instance.")
        .whisper()?;
    Ok(())
}

fn convert_whisper_error_to_io_error() -> Result<(), Error> {
    let whisper = Whisper::new()
        .icon(IconKind::UnicodeCheckMark)
        .message("Converting a `WhisperError` into an `io::Error`.")
        .whisper()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;
    Ok(())
}


fn unwrap() {
    Whisper::new()
        .icon(IconKind::NfFaInfoCircle)
        .message("Unwrapping a `Whisper` instance.")
        .whisper()
        .unwrap();
}

fn unwrap_or_else() {
    Whisper::new()
        .icon(IconKind::NfFaBug)
        .message("Unwrapping a `Whisper` instance or panicking with a custom message.")
        .whisper()
        .unwrap_or_else(|err| panic!("Failed to print message: {}", err));
}

fn expect() {
    Whisper::new()
        .icon(IconKind::NfFaWarning)
        .message("Expecting a `Whisper` instance to be `Ok`.")
        .whisper()
        .expect("Failed to print message");
}

fn map_err_and_propagate_io_error() -> Result<(), Error> {
    Whisper::new()
        .icon(IconKind::NfFaTimes)
        .message("Propagating a `WhisperError` as an `io::Error`.")
        .whisper()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;
    Ok(())
}

fn ok_discard_error_if_any() {
    Whisper::new()
        .icon(IconKind::NfFaTimes)
        .message("// Discarding a `WhisperError` if any occurs.")
        .whisper()


        .ok();
}

fn or_else() -> Result<(), WhisperError> {
    Whisper::new()
        .icon(IconKind::NfFaTimes)
        .message("Handling a `WhisperError` using `or_else`.")
        .whisper()
        .or_else(|err| Err(err))
}

fn box_dyn_error() -> Result<(), Box<dyn std::error::Error>> {
   Whisper::new()
       .icon(IconKind::NfFaTimes)
      .message("using box dyn error")
      .whisper()?;
   Ok(())
}
```
## Custom Error Handling Examples

```rust
use murmur::{Whisper, IconKind, WhisperError};

#[derive(Debug)]
enum CustomError {
  WhisperError(String),
}

impl From<WhisperError> for CustomError {
  fn from(error: WhisperError) -> Self {
    CustomError::WhisperError(format!("We can add more info to the error: {:?}", error))
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

### Using thiserror
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

fn thiserror_error_conversion_example() -> Result<(), CustomError> {
   Whisper::new()
     .icon(IconKind::NfFaTimes)
     .message("Using thiserror")
     .whisper()
     .map_err(CustomError::from)?;
  Ok(())
 }
```

 