use std::io::{self, BufWriter, Error, StdoutLock, Write};

use super::color_map::ColorMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DisplayError {
    #[error("Error writing to buffer: {0}")]
    WriteError(Error),

    #[error("Error flushing buffer: {0}")]
    FlushError(Error),
}

// impl From<Error> for DisplayError {
//     fn from(err: Error) -> Self {
//         Self::WriteError(err)
//     }
// }

impl DisplayError {
    /// Flushes the buffer of the provided `BufWriter`.
    ///
    /// # Arguments
    ///
    /// * `handle`: A mutable reference to a `BufWriter` that is to be flushed.
    ///
    fn flush_buffer(handle: &mut BufWriter<StdoutLock>) -> Result<(), Self> {
        if let Err(err) = handle.flush() {
            #[cfg(feature = "tracing")]
            tracing::error!("Error flushing buffer: {}", err);
            return Err(Self::FlushError(err));
        }
        Ok(())
    }
    fn new_write_error(err: Error) -> Self {
        #[cfg(feature = "tracing")]
        tracing::error!("Error writing to buffer: {}", err);
        Self::WriteError(err)
    }
}

/// The capacity of the buffer used to write to stdout.
const BUFFER_CAPACITY: usize = 8192;

/// Prints a message with a specific color and an optional icon prefix.
///
/// # Arguments
///
/// * `color_map`: A reference to a `ColorMap` that maps color names to functions that colorize a string.
/// * `color`: A string slice that represents the color of the message and the prefix.
/// * `prefix`: A string slice that represents the icon to be printed before the message.
/// * `message`: A string slice that represents the message to be printed.
///
#[cfg_attr(feature = "tracing", tracing::instrument(skip(color_map)))]
pub fn murmur_message(
    color_map: &ColorMap,
    color: &str,
    prefix: &str,
    message: &str,
) -> Result<(), DisplayError> {
    let stdout = io::stdout();
    let mut handle = BufWriter::with_capacity(BUFFER_CAPACITY, stdout.lock());

    if let Some(color_fn) = color_map.map.get(color) {
        writeln!(handle, "{}{}", color_fn(prefix), color_fn(message))
            .map_err(DisplayError::new_write_error)?;

        DisplayError::flush_buffer(&mut handle)?;

        #[cfg(feature = "tracing")]
        tracing::info!("Message displayed with color: {}", color);
    } else {
        writeln!(handle, "{prefix}{message}").map_err(DisplayError::new_write_error)?;

        DisplayError::flush_buffer(&mut handle)?;

        #[cfg(feature = "tracing")]
        tracing::warn!("Message displayed without color");
    }

    Ok(())
}
