# murmur

This Rust crate provides functionality for formatting print messages with `NerdFonts` or `Unicode` icons. 
It includes the `Whisper` struct which represents a collection of messages with an optional `IconKind`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
murmur = "0.1.0"
```

Add this to use the `Whisper` struct and `IconKind` enum:

```rust
use murmur::{Whisper, IconKind};
```

Available `Whisper` methods:

- `new()`: Creates a new `Whisper` instance
- `.icon()`: Adds an icon to the `Whisper` instance
- `.message()`: Adds a message to the `Whisper` instance
- `.message_vec()`: Add a vector of messages to the `Whisper` instance
- `.whisper()`: Builds the `Whisper` instance and prints the messages

## Examples

Here are some examples of how to use the `Whisper` struct:

### Creating a new `Whisper` instance

```rust
use murmur::{Whisper, IconKind};

Whisper::new()
    .icon(IconKind::NfFaCheck)
    .message("message")
    .whisper()
    .unwrap();
```

### Adding an icon to the `Whisper` instance

```rust
use murmur::{Whisper, IconKind};
use std::io::{Error, ErrorKind};

Whisper::new()
    .icon(IconKind::UnicodeCrossMark)
    .message("message")
    .message("Another message")
    .whisper()
    .expect("Failed to print message");
```
### Adding a single message to the `Whisper` instance

```rust
use murmur::Whisper;
use std::io::{Error, ErrorKind};

Whisper::new()
   .message("1 message without icon")
   .message("2 message all messages after the first indents by 2 spaces")
   .message("3 message")
   .whisper()
   .map_err(|err| Error::new(ErrorKind::Other, err))?;
```

This will output:

```
1 message without icon
  2 message without icon indents by 2 spaces all messages after the first
  3 message
```

### Adding multiple messages to the `Whisper` instance

```rust
use murmur::Whisper;

let whisper = Whisper::new()
  .message_vec(vec!["1 message without icon", "2 message", "3 message"])
  .whisper()
  .or_else(|err| panic!("Failed to print message: {err}"));
```

## Error Handling

The `whisper` function returns a `Result`. If the operation is successful, it returns `Ok(())`.
If there is an error during the operation, it returns `WhisperError`.
```rust
use murmur::{Whisper, IconKind, WhisperError};
use std::io::{Error, ErrorKind};

fn create_whisper() -> Result<(), WhisperError> {
    let whisper = Whisper::new()
        .icon(IconKind::NfFaBug)
        .message("test_whisper_creation")
        .whisper()?;
    Ok(())    
}

fn convert_whisper_error_to_io_error() -> Result<(), Error> {
    let whisper = Whisper::new()
        .icon(IconKind::NfFaBug)
        .message("test_whisper_conversion")
        .whisper()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;
    Ok(())
}
```
 