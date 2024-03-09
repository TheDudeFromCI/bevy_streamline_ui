//! Contains blocks related to the background of a node.

use bevy::prelude::*;

use super::DataBlock;
use crate::prelude::{NodeBundleBuilder, NodeBundleType};

/// An enum containing the different ways a texture can be displayed.
#[derive(Debug, Default, Clone)]
pub enum NodeTextureScaling {
    /// The texture is stretched to fit the size of the node.
    #[default]
    Stretched,

    /// The texture is tiled to fit the size of the node.
    Tiled {
        /// Whether to tile the texture on the x-axis.
        tile_x: bool,

        /// Whether to tile the texture on the y-axis.
        tile_y: bool,

        /// The amount a tile will be stretched before repeating.
        stretch_value: f32,
    },

    /// The texture is sliced into a 9-slice grid to fit the size of the node.
    Sliced {
        /// The slicer to use for slicing the texture.
        slicer: TextureSlicer,
    },
}

/// A data block for defining the background of a UI node.
#[derive(Debug, Default, Clone)]
pub enum NodeBackground {
    /// The node does not have a background.
    #[default]
    None,

    /// The node has a solid color background.
    Color {
        /// The color.
        color: Color,
    },

    /// The node has a background image.
    Image {
        /// The background image.
        img: String,

        /// The color tint of the image.
        tint: Color,

        /// The texture scaling mode to use for the image.
        tex_scaling: NodeTextureScaling,
    },
}

impl DataBlock for NodeBackground {
    fn apply_to_node(self, node: &mut NodeBundleBuilder, asset_server: &AssetServer) {
        match self {
            NodeBackground::None => {}
            NodeBackground::Color { color } => {
                let bg_color: BackgroundColor = color.into();
                node.insert(bg_color);
            }
            NodeBackground::Image {
                img,
                tint,
                tex_scaling,
            } => {
                let bg_color: BackgroundColor = tint.into();
                let bg_img: UiImage = asset_server.load(img).into();

                node.bundle_type(NodeBundleType::Image);
                node.insert((bg_img, bg_color));

                match tex_scaling {
                    NodeTextureScaling::Stretched => {}
                    NodeTextureScaling::Tiled {
                        tile_x,
                        tile_y,
                        stretch_value,
                    } => {
                        node.insert(ImageScaleMode::Tiled {
                            tile_x,
                            tile_y,
                            stretch_value,
                        });
                    }

                    NodeTextureScaling::Sliced { slicer } => {
                        node.insert(ImageScaleMode::Sliced(slicer));
                    }
                };
            }
        }
    }
}
