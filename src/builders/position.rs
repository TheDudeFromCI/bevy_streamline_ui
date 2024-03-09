//! A builder for defining the positioning of a [`crate::prelude::UiNode`].

use bevy::prelude::*;

use crate::prelude::{AnchorPoint, NodePosition};

/// A builder for defining the positioning of a [`crate::prelude::UiNode`].
#[derive(Debug, Default, Clone)]
pub struct PositionBuilder;

impl PositionBuilder {
    /// Sets the position of the node to be relative to it's parent.
    ///
    /// This will allow the parent to define how this node should be positioned.
    pub fn relative() -> RelativePositionBuilder {
        RelativePositionBuilder::default()
    }

    /// Sets the position of the node to be absolute.
    pub fn absolute() -> AbsolutePositionBuilder {
        AbsolutePositionBuilder::default()
    }

    /// Sets the position of the node to be anchored to it's parent using a
    /// docking method.
    pub fn anchored(anchor: AnchorPoint) -> AnchoredPositionBuilder {
        AnchoredPositionBuilder {
            anchor,
            ..default()
        }
    }
}

/// A builder for defining the relative positioning of a node.
#[derive(Debug, Default, Clone)]
pub struct RelativePositionBuilder {
    /// The width of the entity relative to it's parent.
    width: Val,

    /// The height of the entity relative to it's parent.
    height: Val,
}

impl RelativePositionBuilder {
    /// Sets the size of the node.
    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl From<RelativePositionBuilder> for NodePosition {
    fn from(value: RelativePositionBuilder) -> Self {
        NodePosition::Relative {
            width: value.width,
            height: value.height,
        }
    }
}

/// A builder for defining the absolute positioning of a node.
#[derive(Debug, Default, Clone)]
pub struct AbsolutePositionBuilder {
    /// The x position of the entity relative to it's parent.
    x: Val,

    /// The y position of the entity relative to it's parent.
    y: Val,

    /// The width of the entity relative to it's parent.
    width: Val,

    /// The height of the entity relative to it's parent.
    height: Val,
}

impl AbsolutePositionBuilder {
    /// Sets the position of the node.
    pub fn pos(mut self, x: Val, y: Val) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the size of the node.
    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets the node to completely fill the parent node.
    pub fn full_size(mut self) -> Self {
        self.x = Val::Px(0.0);
        self.y = Val::Px(0.0);
        self.width = Val::Percent(100.0);
        self.height = Val::Percent(100.0);
        self
    }
}

impl From<AbsolutePositionBuilder> for NodePosition {
    fn from(builder: AbsolutePositionBuilder) -> Self {
        NodePosition::Absolute {
            x: builder.x,
            y: builder.y,
            width: builder.width,
            height: builder.height,
        }
    }
}

/// A builder for defining the anchored positioning of a node.
#[derive(Debug, Default, Clone)]
pub struct AnchoredPositionBuilder {
    /// The anchor point to use for positioning.
    anchor: AnchorPoint,

    /// The width of the entity relative to it's parent.
    width: Val,

    /// The height of the entity relative to it's parent.
    height: Val,

    /// The space between this entity and the border of it's parent.
    margin: Val,
}

impl AnchoredPositionBuilder {
    /// Sets the anchor point of the node.
    pub fn anchor(mut self, anchor: AnchorPoint) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets the size of the node.
    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets the margin of the node.
    pub fn margin(mut self, margin: Val) -> Self {
        self.margin = margin;
        self
    }
}

impl From<AnchoredPositionBuilder> for NodePosition {
    fn from(builder: AnchoredPositionBuilder) -> Self {
        NodePosition::Anchored {
            anchor: builder.anchor,
            width: builder.width,
            height: builder.height,
            margin: builder.margin,
        }
    }
}
