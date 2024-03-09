//! This module provides utilities for anchoring UI elements to their parent
//! elements.

use bevy::prelude::*;

use super::DataBlock;
use crate::prelude::NodeBundleBuilder;

/// Defines the anchor point for a UI element relative to its parent.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnchorPoint {
    /// The top-left corner of the parent element.
    TopLeft,

    /// The top-center of the parent element.
    TopCenter,

    /// The top-right corner of the parent element.
    TopRight,

    /// The center-left of the parent element.
    CenterLeft,

    /// The center of the parent element.
    #[default]
    Center,

    /// The center-right of the parent element.
    CenterRight,

    /// The bottom-left corner of the parent element.
    BottomLeft,

    /// The bottom-center of the parent element.
    BottomCenter,

    /// The bottom-right corner of the parent element.
    BottomRight,
}

/// A data block for defining how a node is anchored to it's parent.
#[derive(Debug, Clone)]
pub enum NodePosition {
    /// Allow this node to be positioned by it's parent container.
    Relative {
        /// The width of the entity relative to it's parent.
        width: Val,

        /// The height of the entity relative to it's parent.
        height: Val,
    },

    /// Position this node absolutely within it's parent container.
    Absolute {
        /// The x position of the entity relative to it's parent.
        x: Val,

        /// The y position of the entity relative to it's parent.
        y: Val,

        /// The width of the entity relative to it's parent.
        width: Val,

        /// The height of the entity relative to it's parent.
        height: Val,
    },

    /// Position this node absolutely within it's parent container using an
    /// anchor point.
    Anchored {
        /// The anchor point to use for positioning.
        anchor: AnchorPoint,

        /// The width of the entity relative to it's parent.
        width: Val,

        /// The height of the entity relative to it's parent.
        height: Val,

        /// The space between this entity and the border of it's parent.
        ///
        /// Note that using `Val::Auto` will not work as expected.
        margin: Val,
    },
}

impl Default for NodePosition {
    fn default() -> Self {
        NodePosition::Relative {
            width: Val::Auto,
            height: Val::Auto,
        }
    }
}

impl DataBlock for NodePosition {
    fn apply_to_node(self, node: &mut NodeBundleBuilder, _: &AssetServer) {
        let style = node.get_style_mut();

        match self {
            NodePosition::Relative { width, height } => {
                style.width = width;
                style.height = height;
            }
            NodePosition::Absolute {
                x,
                y,
                width,
                height,
            } => {
                style.position_type = PositionType::Absolute;
                style.left = x;
                style.top = y;
                style.width = width;
                style.height = height;
            }
            NodePosition::Anchored {
                anchor,
                width,
                height,
                margin,
            } => {
                set_anchor_point(style, anchor, margin);
                style.width = width;
                style.height = height;
            }
        };
    }
}

/// Modifies the given style to position the element according to the given
/// anchor point and margin.
fn set_anchor_point(style: &mut Style, point: AnchorPoint, margin: Val) {
    style.margin = UiRect::all(margin);

    match point {
        AnchorPoint::TopLeft => {
            style.position_type = PositionType::Absolute;
            style.top = Val::Px(0.0);
            style.bottom = Val::Auto;
            style.left = Val::Px(0.0);
            style.right = Val::Auto;
        }
        AnchorPoint::TopCenter => {
            style.position_type = PositionType::Absolute;
            style.top = Val::Px(0.0);
            style.bottom = Val::Auto;
            style.left = Val::Px(0.0);
            style.right = Val::Px(0.0);
            style.margin = center_margin_hor(style.margin);
        }
        AnchorPoint::TopRight => {
            style.position_type = PositionType::Absolute;
            style.top = Val::Px(0.0);
            style.bottom = Val::Auto;
            style.right = Val::Px(0.0);
            style.left = Val::Auto;
        }
        AnchorPoint::CenterLeft => {
            style.position_type = PositionType::Absolute;
            style.top = Val::Px(0.0);
            style.bottom = Val::Px(0.0);
            style.left = Val::Px(0.0);
            style.right = Val::Auto;
            style.margin = center_margin_ver(style.margin);
        }
        AnchorPoint::Center => {
            style.position_type = PositionType::Absolute;
            style.top = Val::Px(0.0);
            style.bottom = Val::Px(0.0);
            style.left = Val::Px(0.0);
            style.right = Val::Px(0.0);
            style.margin = center_margin();
        }
        AnchorPoint::CenterRight => {
            style.position_type = PositionType::Absolute;
            style.top = Val::Px(0.0);
            style.bottom = Val::Px(0.0);
            style.left = Val::Auto;
            style.right = Val::Px(0.0);
            style.margin = center_margin_ver(style.margin);
        }
        AnchorPoint::BottomLeft => {
            style.position_type = PositionType::Absolute;
            style.top = Val::Auto;
            style.bottom = Val::Px(0.0);
            style.left = Val::Px(0.0);
            style.right = Val::Auto;
        }
        AnchorPoint::BottomCenter => {
            style.position_type = PositionType::Absolute;
            style.top = Val::Auto;
            style.bottom = Val::Px(0.0);
            style.left = Val::Px(0.0);
            style.right = Val::Px(0.0);
            style.margin = center_margin_hor(style.margin);
        }
        AnchorPoint::BottomRight => {
            style.position_type = PositionType::Absolute;
            style.top = Val::Auto;
            style.bottom = Val::Px(0.0);
            style.left = Val::Auto;
            style.right = Val::Px(0.0);
        }
    }
}

/// Takes in a UiRect and returns a new UiRect with the top and bottom fields
/// set to `Auto`.
///
/// This is useful for centering an element horizontally.
fn center_margin_hor(margin: UiRect) -> UiRect {
    UiRect {
        top: margin.top,
        bottom: margin.bottom,
        left: Val::Auto,
        right: Val::Auto,
    }
}

/// Takes in a UiRect and returns a new UiRect with the left and right fields
/// set to `Auto`.
///
/// This is useful for centering an element vertically.
fn center_margin_ver(margin: UiRect) -> UiRect {
    UiRect {
        top: Val::Auto,
        bottom: Val::Auto,
        left: margin.left,
        right: margin.right,
    }
}

/// Returns a new UiRect with all fields set to `Auto`.
///
/// This is useful for centering an element both horizontally and vertically.
fn center_margin() -> UiRect {
    UiRect {
        top: Val::Auto,
        bottom: Val::Auto,
        left: Val::Auto,
        right: Val::Auto,
    }
}
