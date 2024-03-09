//! A builder for defining how text is displayed within a node.

use bevy::prelude::*;
use bevy::text::BreakLineOn;

use crate::prelude::{AnchorPoint, NodeText, NodeTextSection};

/// A builder for defining how text is displayed within a node.
#[derive(Debug, Clone)]
pub struct TextBuilder {
    /// The anchor point for the text.
    anchor_point: AnchorPoint,

    /// The sections of the text.
    sections: Vec<TextSectionBuilder>,

    /// The line break behavior for the text.
    line_break: BreakLineOn,
}

impl TextBuilder {
    /// Sets the anchor point for the text.
    pub fn anchor_point(mut self, anchor_point: AnchorPoint) -> Self {
        self.anchor_point = anchor_point;
        self
    }

    /// Adds a section to the text.
    pub fn section<T: Into<TextSectionBuilder>>(mut self, section: T) -> Self {
        self.sections.push(section.into());
        self
    }

    /// Sets the line break behavior for the text.
    pub fn line_break(mut self, line_break: BreakLineOn) -> Self {
        self.line_break = line_break;
        self
    }
}

impl Default for TextBuilder {
    fn default() -> Self {
        Self {
            anchor_point: Default::default(),
            sections: Default::default(),
            line_break: BreakLineOn::WordBoundary,
        }
    }
}

impl From<TextBuilder> for NodeText {
    fn from(value: TextBuilder) -> Self {
        Self {
            anchor_point: value.anchor_point,
            sections: value.sections.into_iter().map(Into::into).collect(),
            line_break: value.line_break,
        }
    }
}

/// A builder for defining a section of text.
#[derive(Debug, Clone)]
pub struct TextSectionBuilder {
    /// The text to display.
    text: String,

    /// The font to use for the text.
    font: String,

    /// The size of the text.
    text_size: f32,

    /// The color of the text.
    color: Color,
}

impl TextSectionBuilder {
    /// Creates a new text section builder.
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            font: Default::default(),
            text_size: 16.0,
            color: Color::BLACK,
        }
    }

    /// Sets the font to use for the text.
    pub fn font(mut self, font: &str) -> Self {
        self.font = font.to_string();
        self
    }

    /// Sets the size of the text.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = text_size;
        self
    }

    /// Sets the color of the text.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl From<TextSectionBuilder> for NodeTextSection {
    fn from(builder: TextSectionBuilder) -> Self {
        Self {
            text: builder.text,
            font: builder.font,
            text_size: builder.text_size,
            color: builder.color,
        }
    }
}
