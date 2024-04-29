//! A builder for defining the text field of a [`crate::prelude::UiNode`].

use bevy::prelude::*;
use bevy::text::BreakLineOn;

use crate::prelude::{AnchorPoint, NodeTextField};

/// A builder for defining a text field.
#[derive(Debug, Clone)]
pub struct TextFieldBuilder {
    /// The font to use for the text.
    font: String,

    /// The size of the text.
    font_size: f32,

    /// The color of the text.
    color: Color,

    /// The maximum number of characters that can be entered, if any.
    max_chars: Option<usize>,

    /// Whether or not the text field may have multiple lines.
    single_line: bool,

    /// The default text to display when the field is empty.
    placeholder: Option<String>,

    /// The color of the placeholder text.
    placeholder_color: Color,

    /// The anchor point for the text.
    anchor_point: AnchorPoint,

    /// The line break behavior for the text.
    line_break: BreakLineOn,
}

impl Default for TextFieldBuilder {
    fn default() -> Self {
        Self {
            font: Default::default(),
            font_size: 16.0,
            color: Color::BLACK,
            max_chars: None,
            single_line: false,
            placeholder: Default::default(),
            placeholder_color: Color::GRAY,
            anchor_point: AnchorPoint::CenterLeft,
            line_break: BreakLineOn::WordBoundary,
        }
    }
}

impl TextFieldBuilder {
    /// Sets the font to use for the text.
    pub fn font<S: Into<String>>(mut self, font: S) -> Self {
        self.font = font.into();
        self
    }

    /// Sets the size of the text.
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Sets the color of the text.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the maximum number of characters that can be entered.
    pub fn max_chars(mut self, max_chars: usize) -> Self {
        self.max_chars = Some(max_chars);
        self
    }

    /// Sets the text to only allow a single line of input.
    pub fn single_line(mut self) -> Self {
        self.single_line = true;
        self
    }

    /// Sets the text to allow multiple lines of input.
    pub fn multiple_lines(mut self) -> Self {
        self.single_line = false;
        self
    }

    /// Sets the default text to display when the field is empty.
    pub fn placeholder_text<S: Into<String>>(mut self, placeholder: S) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the color of the placeholder text.
    pub fn placeholder_color(mut self, color: Color) -> Self {
        self.placeholder_color = color;
        self
    }

    /// Sets the anchor point for the text.
    pub fn anchor_point(mut self, anchor_point: AnchorPoint) -> Self {
        self.anchor_point = anchor_point;
        self
    }

    /// Sets the line break behavior for the text.
    pub fn line_break(mut self, line_break: BreakLineOn) -> Self {
        self.line_break = line_break;
        self
    }
}

impl From<TextFieldBuilder> for NodeTextField {
    fn from(value: TextFieldBuilder) -> Self {
        Self {
            font: value.font,
            font_size: value.font_size,
            color: value.color,
            max_chars: value.max_chars,
            single_line: value.single_line,
            placeholder: value.placeholder,
            placeholder_color: value.placeholder_color,
            anchor_point: value.anchor_point,
            line_break: value.line_break,
        }
    }
}
