//! This module contains the nodes that are used to build the UI.
//!
//! A node is a single element in the UI hierarchy. It can be a panel, a button,
//! a text label, etc. Nodes may contain other nodes, forming a tree structure
//! that represents the UI.
//!
//! Nodes are only a building block for the UI, and are consumed when the UI is
//! built.

use bevy::prelude::*;

use crate::prelude::{
    DataBlock,
    NodeBackground,
    NodeBundleBuilder,
    NodeChildren,
    NodePosition,
    NodeText,
    NodeTextField,
};

pub mod text_field;

/// A trait for UI node builders that can be built into entities.
#[derive(Debug, Clone)]
pub enum UiNode {
    /// A canvas node is a invisible, full-screen node designed to act as a root
    /// node for a UI hierarchy.
    Canvas {
        /// The children of the canvas.
        children: NodeChildren,
    },

    /// A panel is a standard container node.
    Panel {
        /// The background of the panel.
        background: NodeBackground,

        /// The position of the panel.
        position: NodePosition,

        /// The children of the panel.
        children: NodeChildren,
    },

    /// A text node is a node that contains text.
    Text {
        /// The background of the text.
        background: NodeBackground,

        /// The position of the text.
        position: NodePosition,

        /// The text data for the text.
        text: NodeText,
    },

    /// A text field node is a node that contains a text field.
    TextField {
        /// The background of the text field.
        background: NodeBackground,

        /// The position of the text field.
        position: NodePosition,

        /// The text field data for the text field.
        text_field: NodeTextField,
    },
}

impl UiNode {
    /// Consumes this [`UiNode`] and creates a new UI entity hierarchy.
    pub fn build(self, cmd: &mut Commands, asset_server: &AssetServer) {
        self.build_node(cmd, asset_server, None);
    }

    /// Consumes this [`UiNode`] and creates a new UI entity hierarchy with an
    /// optional parent.
    pub(crate) fn build_node(
        self,
        cmd: &mut Commands,
        asset_server: &AssetServer,
        parent: Option<Entity>,
    ) {
        match self {
            UiNode::Canvas { children } => {
                let mut node = NodeBundleBuilder::default();
                node.set_parent(parent);

                let style = node.get_style_mut();
                style.top = Val::Px(0.0);
                style.left = Val::Px(0.0);
                style.width = Val::Percent(100.0);
                style.height = Val::Percent(100.0);

                children.apply_to_node(&mut node, asset_server);
                node.build(cmd, asset_server);
            }

            UiNode::Panel {
                background,
                position,
                children,
            } => {
                let mut node = NodeBundleBuilder::default();
                node.set_parent(parent);

                background.apply_to_node(&mut node, asset_server);
                position.apply_to_node(&mut node, asset_server);
                children.apply_to_node(&mut node, asset_server);
                node.build(cmd, asset_server);
            }

            UiNode::Text {
                background,
                position,
                text,
            } => {
                let mut container_node = NodeBundleBuilder::default();
                container_node.set_parent(parent);

                background.apply_to_node(&mut container_node, asset_server);
                position.apply_to_node(&mut container_node, asset_server);
                text.apply_to_parent(&mut container_node, asset_server);
                let container_id = container_node.build(cmd, asset_server);

                let mut text_node = NodeBundleBuilder::default();
                text_node.set_parent(Some(container_id));

                text.apply_to_node(&mut text_node, asset_server);
                text_node.build(cmd, asset_server);
            }

            UiNode::TextField {
                background,
                position,
                text_field,
            } => {
                let mut container_node = NodeBundleBuilder::default();
                container_node.set_parent(parent);

                background.apply_to_node(&mut container_node, asset_server);
                position.apply_to_node(&mut container_node, asset_server);
                text_field.apply_to_parent(&mut container_node, asset_server);
                let container_id = container_node.build(cmd, asset_server);

                let mut text_field_node = NodeBundleBuilder::default();
                text_field_node.set_parent(Some(container_id));

                text_field.apply_to_node(&mut text_field_node, asset_server);
                text_field_node.build(cmd, asset_server);
            }
        }
    }
}
