 ## Customizing Error Handling
 
 ```rust
 use murmur::{Whisper, IconKind, WhisperError};

 #[derive(Debug)]
 enum CustomError {
     WhisperError(String),
 }

 impl From<WhisperError> for CustomError {
     fn from(error: WhisperError) -> Self {
         Self::WhisperError(format("We can add more info to the error: {error}"))
     }
 }

 fn explicit_closure_for_error_conversion() -> Result<(), CustomError> {
     Whisper::new()
         .icon(IconKind::NfFaTimes)
         .message("Explicit closure to convert a `WhisperError` into a `CustomError`.")
         .whisper()
         .map_err(|err| CustomError::from(err))?;
     Ok(())
 }

 fn function_reference_for_error_conversion() -> Result<(), CustomError> {
     Whisper::new()
         .icon(IconKind::NfFaTimes)
         .message("Function reference to convert a `WhisperError` into a `CustomError`.")
         .whisper()
         .map_err(CustomError::from)?;
     Ok(())
 }
 ```

