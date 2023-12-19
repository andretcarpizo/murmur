//! This module contains functionality to print messages to stdout based on a color hash map.

use owo_colors::OwoColorize;
use std::collections::HashMap;

type ColorFunction = Box<dyn Fn(&str) -> String>;
type ColorMapType = HashMap<&'static str, ColorFunction>;

#[derive(Default)]
pub struct ColorMap {
    pub map: ColorMapType,
}

impl ColorMap {
    pub fn new() -> Self {
        let mut map: ColorMapType = HashMap::new();
        map.insert("red", Box::new(|text| text.red().to_string()));
        map.insert("green", Box::new(|text| text.green().to_string()));
        map.insert("white", Box::new(|text| text.white().to_string()));
        map.insert("cyan", Box::new(|text| text.cyan().to_string()));
        map.insert("yellow", Box::new(|text| text.yellow().to_string()));

        Self { map }
    }
}
