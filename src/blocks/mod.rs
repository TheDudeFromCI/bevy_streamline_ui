//! This module contains data blocks that can be constructed from builders. They
//! are used within nodes to define how that node should be displayed.

use bevy::asset::AssetServer;

mod background;
mod children;
mod position;
mod text;

pub use background::*;
pub use children::*;
pub use position::*;
pub use text::*;

use crate::prelude::NodeBundleBuilder;

/// A data block that can be applied to a node to define how the node bundle
/// should be constructed.
pub trait DataBlock {
    /// Writes the data defined by the block onto the given node.
    fn apply_to_node(self, node: &mut NodeBundleBuilder, asset_server: &AssetServer);

    /// If this data block is meant to be applied to both a parent node and a
    /// child node, this method should be implemented to apply the data to the
    /// parent node. This function is a no-op by default.
    fn apply_to_parent(&self, _: &mut NodeBundleBuilder, _: &AssetServer) {}
}
