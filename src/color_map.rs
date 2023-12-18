//! This module contains functionality to print messages to stdout based on a color hash map.

use std::collections::HashMap;
use owo_colors::OwoColorize;
use std::io::{self, Write};

/// A type alias for a `HashMap` that maps color names to functions that colorize a string.
type ColorMap = HashMap<&'static str, Box<dyn Fn(&str) -> String>>;

/// Creates a color map that maps color names to functions that colorize a string.
///
/// # Returns
///
/// A `ColorMap`
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn initialize() -> ColorMap {
    let mut color_map: ColorMap = HashMap::new();
    color_map.insert("red", Box::new(|text| text.red().to_string()));
    color_map.insert("green", Box::new(|text| text.green().to_string()));
    color_map.insert("white", Box::new(|text| text.white().to_string()));
    color_map.insert("cyan", Box::new(|text| text.cyan().to_string()));
    color_map.insert("yellow", Box::new(|text| text.yellow().to_string()));

    #[cfg(feature = "tracing")]
    tracing::info!("COLOR_MAP initialized");

    color_map
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
pub fn display_message(color_map: &ColorMap, color: &str, prefix: &str, message: &str) -> Result<(), io::Error> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::with_capacity(BUFFER_CAPACITY, stdout.lock());

    if let Some(color_fn) = color_map.get(color) {
        if let Err(err) = writeln!(handle, "{}{}", color_fn(prefix), color_fn(message)) {
            handle_error("Error writing to buffer", &err);
            return Err(err);
        }

        flush_buffer(&mut handle);

        #[cfg(feature = "tracing")]
        tracing::info!("Message displayed with color: {}", color);

        Ok(())
    } else {
        if let Err(err) = writeln!(handle, "{prefix}{message}") {
            handle_error("Error writing to buffer", &err);
            return Err(err);
        }

        flush_buffer(&mut handle);

        #[cfg(feature = "tracing")]
        tracing::error!("Failed to get color function for color: {}", color);

        Ok(())
    }
}

/// Flushes the buffer of the provided `BufWriter`.
///
/// # Arguments
///
/// * `handle`: A mutable reference to a `BufWriter` that is to be flushed.
///
fn flush_buffer(handle: &mut io::BufWriter<io::StdoutLock>) {
    if let Err(err) = handle.flush() {
        eprintln!("Error flushing buffer: {}", &err);
        #[cfg(feature = "tracing")]
        tracing::error!("Error flushing buffer: {}", err);
    }
}

/// Handles an error by printing an error message and the error to stderr.
///
/// # Arguments
///
/// * `msg`: A string slice that represents the error message to be printed.
/// * `err`: A reference to an `io::Error` that is to be printed.
///
fn handle_error(msg: &str, err: &io::Error) {
    eprintln!("{}: {}", msg, err);
    #[cfg(feature = "tracing")]
    tracing::error!("{}: {}", msg, err);
}
