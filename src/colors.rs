//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}{}{} {} {}", red("Red"), green("Green"), blue("Blue"), cyan("Cyan"), bold("Bold"));
//! ```

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", red("Red"));
/// println!("This is a {} string", red("red"));
/// println!("This is a {} string", cyan("cyan"));
/// println!()
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

pub fn cyan(s: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", s)
}

pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// Represents available colors and styles for text formatting in the terminal.
///
/// This enumeration is used by the `ColorString` struct to apply the desired color or style.
///
/// # Examples
///
/// ```
/// use cli_utils::colors::Color;
///
/// let color = Color::Red;
/// // Use `color` to set the text color.
/// ```
pub enum Color {
    /// Red text.
    Red,
    /// Green text.
    Green,
    /// Blue text.
    Blue,
    /// Bold text.
    Bold,
    /// Cyan text.
    Cyan,
}

/// A structure that stores a text (`String`) and an associated color or style (`Color`).
///
/// `ColorString` allows combining a text with a specific color or style and generating
/// a formatted version with ANSI codes for display in the terminal.
/// # Examples
/// ```
/// use cli_utils::colors::{ColorString, Color};
///
/// let mut color_str = ColorString {
///     color: Color::Green,
///     string: String::from("Green Text"),
///     colorized: String::new(),
/// };
/// color_str.paint();
/// println!("{}", color_str.colorized);
/// ```
pub struct ColorString {
    /// The color or style to be applied to the text.
    pub color: Color,
    /// The original unformatted text.
    pub string: String,
    /// The text formatted with the applied color or style.
    pub colorized: String,
}

impl ColorString {
    // create a method that will use the string and color fields to create a colorized string and
    // assign it to the colorized field
    /// Applies the color or style specified in the `color` field to the text.
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
            Color::Cyan => self.colorized = cyan(&self.string),
        };
    }

    /// Resets the colorized text to the original format.
    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }
}
