//! Contains blocks related to the text in a node.

use bevy::prelude::*;
use bevy::text::BreakLineOn;

use super::{AnchorPoint, DataBlock};
use crate::prelude::{NodeBundleBuilder, NodeBundleType};

/// Defines a section of text.
#[derive(Debug, Default, Clone)]
pub struct NodeTextSection {
    /// The text to display.
    pub text: String,

    /// The font to use for the text.
    pub font: String,

    /// The size of the text.
    pub text_size: f32,

    /// The color of the text.
    pub color: Color,
}

/// Defines the text for a node.
#[derive(Debug, Clone)]
pub struct NodeText {
    /// The anchor point for the text.
    pub anchor_point: AnchorPoint,

    /// The sections of the text.
    pub sections: Vec<NodeTextSection>,

    /// The line break behavior for the text.
    pub line_break: BreakLineOn,
}

impl Default for NodeText {
    fn default() -> Self {
        Self {
            anchor_point: Default::default(),
            sections: Default::default(),
            line_break: BreakLineOn::WordBoundary,
        }
    }
}

impl DataBlock for NodeText {
    fn apply_to_node(self, node: &mut NodeBundleBuilder, asset_server: &AssetServer) {
        node.bundle_type(NodeBundleType::Text);

        let mut text = Text::default();
        text.linebreak_behavior = self.line_break;
        text.sections = self
            .sections
            .into_iter()
            .map(|section| TextSection {
                value: section.text,
                style: TextStyle {
                    font: asset_server.load(&section.font),
                    font_size: section.text_size,
                    color: section.color,
                },
            })
            .collect();

        text.justify = match self.anchor_point {
            AnchorPoint::TopLeft => JustifyText::Left,
            AnchorPoint::TopCenter => JustifyText::Center,
            AnchorPoint::TopRight => JustifyText::Right,
            AnchorPoint::CenterLeft => JustifyText::Left,
            AnchorPoint::Center => JustifyText::Center,
            AnchorPoint::CenterRight => JustifyText::Right,
            AnchorPoint::BottomLeft => JustifyText::Left,
            AnchorPoint::BottomCenter => JustifyText::Center,
            AnchorPoint::BottomRight => JustifyText::Right,
        };

        node.insert(text);
    }

    fn apply_to_parent(&self, node: &mut NodeBundleBuilder, _: &AssetServer) {
        let content_alignment = match self.anchor_point {
            AnchorPoint::TopLeft => (AlignContent::FlexStart, JustifyContent::Start),
            AnchorPoint::TopCenter => (AlignContent::FlexStart, JustifyContent::Center),
            AnchorPoint::TopRight => (AlignContent::FlexStart, JustifyContent::End),
            AnchorPoint::CenterLeft => (AlignContent::Center, JustifyContent::Start),
            AnchorPoint::Center => (AlignContent::Center, JustifyContent::Center),
            AnchorPoint::CenterRight => (AlignContent::Center, JustifyContent::End),
            AnchorPoint::BottomLeft => (AlignContent::FlexEnd, JustifyContent::Start),
            AnchorPoint::BottomCenter => (AlignContent::FlexEnd, JustifyContent::Center),
            AnchorPoint::BottomRight => (AlignContent::FlexEnd, JustifyContent::End),
        };

        let item_alignment = match self.anchor_point {
            AnchorPoint::TopLeft => (AlignItems::FlexStart, JustifyItems::Start),
            AnchorPoint::TopCenter => (AlignItems::FlexStart, JustifyItems::Center),
            AnchorPoint::TopRight => (AlignItems::FlexStart, JustifyItems::End),
            AnchorPoint::CenterLeft => (AlignItems::Center, JustifyItems::Start),
            AnchorPoint::Center => (AlignItems::Center, JustifyItems::Center),
            AnchorPoint::CenterRight => (AlignItems::Center, JustifyItems::End),
            AnchorPoint::BottomLeft => (AlignItems::FlexEnd, JustifyItems::Start),
            AnchorPoint::BottomCenter => (AlignItems::FlexEnd, JustifyItems::Center),
            AnchorPoint::BottomRight => (AlignItems::FlexEnd, JustifyItems::End),
        };

        let style = node.get_style_mut();
        (style.align_content, style.justify_content) = content_alignment;
        (style.align_items, style.justify_items) = item_alignment;
    }
}
