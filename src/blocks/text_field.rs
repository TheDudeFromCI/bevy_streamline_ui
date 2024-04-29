//! Contains blocks related to the text field of a node.

use bevy::prelude::*;
use bevy::text::BreakLineOn;

use super::{AnchorPoint, DataBlock};
use crate::prelude::text_field::TextField;
use crate::prelude::{NodeBundleBuilder, NodeBundleType};

/// Defines a text field for a node.
#[derive(Debug, Clone)]
pub struct NodeTextField {
    /// The font to use for the text.
    pub font: String,

    /// The size of the text.
    pub font_size: f32,

    /// The color of the text.
    pub color: Color,

    /// The maximum number of characters that can be entered.
    pub max_chars: Option<usize>,

    /// Whether or not the text field may have multiple lines.
    pub single_line: bool,

    /// The default text to display when the field is empty.
    pub placeholder: Option<String>,

    /// The color of the placeholder text.
    pub placeholder_color: Color,

    /// The anchor point for the text.
    pub anchor_point: AnchorPoint,

    /// The line break behavior for the text.
    pub line_break: BreakLineOn,
}

impl Default for NodeTextField {
    fn default() -> Self {
        Self {
            font: Default::default(),
            font_size: 16.0,
            color: Color::BLACK,
            max_chars: None,
            single_line: false,
            placeholder: None,
            placeholder_color: Color::GRAY,
            anchor_point: AnchorPoint::CenterLeft,
            line_break: BreakLineOn::WordBoundary,
        }
    }
}

impl DataBlock for NodeTextField {
    fn apply_to_node(self, node: &mut NodeBundleBuilder, asset_server: &AssetServer) {
        node.bundle_type(NodeBundleType::Text);

        node.insert(Text {
            linebreak_behavior: self.line_break,
            justify: match self.anchor_point {
                AnchorPoint::TopLeft => JustifyText::Left,
                AnchorPoint::TopCenter => JustifyText::Center,
                AnchorPoint::TopRight => JustifyText::Right,
                AnchorPoint::CenterLeft => JustifyText::Left,
                AnchorPoint::Center => JustifyText::Center,
                AnchorPoint::CenterRight => JustifyText::Right,
                AnchorPoint::BottomLeft => JustifyText::Left,
                AnchorPoint::BottomCenter => JustifyText::Center,
                AnchorPoint::BottomRight => JustifyText::Right,
            },
            ..default()
        });

        node.insert(TextField {
            font: asset_server.load(&self.font),
            font_size: self.font_size,
            font_color: self.color,
            placeholder_text: self.placeholder,
            placeholder_color: self.placeholder_color,
            ..default()
        });
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
