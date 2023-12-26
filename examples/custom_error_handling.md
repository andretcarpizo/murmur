 ## Customizing Error Handling

  ```rust
use murmur::{Whisper, IconKind, WhisperError};

#[derive(Debug)]
enum ErrorKind {
    Whisper(WhisperError),
}

impl From<WhisperError> for ErrorKind {
    fn from(error: WhisperError) -> Self {
        Self::Whisper(error)
    }
}

type Result<T> = std::result::Result<T, ErrorKind>;

fn whisper_message(whisper: &mut Whisper, message: &str) -> Result<()> {
    whisper
        .icon(IconKind::NfFaTimes)
        .message(message)
        .whisper()
        .map_err(ErrorKind::from)?;
    Ok(())
}

fn whisper_with_error_conversion(message: &str) -> Result<()> {
    let mut whisperer = Whisper::new();
    whisper_message(&mut whisperer, message)
}
 ```

