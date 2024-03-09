//! Contains blocks related to the children of a node.

use bevy::prelude::*;

use super::DataBlock;
use crate::prelude::{NodeBundleBuilder, UiNode};

/// A data block for defining the children of a UI node.
#[derive(Debug, Default, Clone)]
pub struct NodeChildren {
    /// A list of child nodes to add to the node.
    pub children: Vec<UiNode>,
}

impl DataBlock for NodeChildren {
    fn apply_to_node(self, node: &mut NodeBundleBuilder, _: &AssetServer) {
        node.set_children(self.children.clone());
    }
}

impl From<Vec<UiNode>> for NodeChildren {
    fn from(children: Vec<UiNode>) -> Self {
        NodeChildren { children }
    }
}
