//! A builder for defining how an image texture is scaled.

use bevy::prelude::*;

use crate::prelude::NodeTextureScaling;

/// A builder for defining how an image texture is scaled.
#[derive(Debug, Default, Clone)]
pub struct TexScalingBuilder;

impl TexScalingBuilder {
    /// Sets the texture to be stretched across the node.
    pub fn stretch(self) -> TexStretchBuilder {
        TexStretchBuilder
    }

    /// Sets the texture to be tiled across the node.
    pub fn tiled(self) -> TexTileBuilder {
        TexTileBuilder::default()
    }

    /// Sets the texture to use a 9-slice scaling mode.
    pub fn sliced(self) -> TexSlicedBuilder {
        TexSlicedBuilder::default()
    }
}

/// A builder for defining how an image texture is stretched.
#[derive(Debug, Default, Clone)]
pub struct TexStretchBuilder;

impl From<TexStretchBuilder> for NodeTextureScaling {
    fn from(_: TexStretchBuilder) -> Self {
        NodeTextureScaling::Stretched
    }
}

/// A builder for defining how an image texture is tiled across a node.
#[derive(Debug, Clone)]
pub struct TexTileBuilder {
    /// Should the image repeat horizontally?
    tile_x: bool,

    /// Should the image repeat vertically?
    tile_y: bool,

    /// Assigns how much a tile is allowed to stretch before being repeated.
    stretch_value: f32,
}

impl TexTileBuilder {
    /// Sets the texture to only be tiled horizontally.
    pub fn tile_horizontally(mut self) -> Self {
        self.tile_x = true;
        self.tile_y = false;
        self
    }

    /// Sets the texture to only be tiled vertically.
    pub fn tile_vertically(mut self) -> Self {
        self.tile_x = false;
        self.tile_y = true;
        self
    }

    /// Sets the texture to be tiled both horizontally and vertically.
    pub fn tile_both(mut self) -> Self {
        self.tile_x = true;
        self.tile_y = true;
        self
    }

    /// Sets how much a tile is allowed to stretch before being repeated.
    pub fn stretch_value(mut self, stretch_value: f32) -> Self {
        self.stretch_value = stretch_value;
        self
    }
}

impl Default for TexTileBuilder {
    fn default() -> Self {
        Self {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.0,
        }
    }
}

impl From<TexTileBuilder> for NodeTextureScaling {
    fn from(value: TexTileBuilder) -> Self {
        NodeTextureScaling::Tiled {
            tile_x: value.tile_x,
            tile_y: value.tile_y,
            stretch_value: value.stretch_value,
        }
    }
}

/// A builder for defining how an image texture is scaled using 9-slice scaling.
#[derive(Debug, Default, Clone)]
pub struct TexSlicedBuilder {
    /// The slicer settings to use for the 9-slice scaling.
    slicer: TextureSlicer,
}

impl TexSlicedBuilder {
    /// Sets the border size uniformly for the 9-slice scaling.
    ///
    /// All corners and edges will have the same border size.
    pub fn border(mut self, border: f32) -> Self {
        self.slicer.border = border.into();
        self
    }

    /// Sets the border size for the top edge of the 9-slice scaling.
    ///
    /// This method will override only the top edge of the border.
    pub fn top_border(mut self, top: f32) -> Self {
        self.slicer.border.top = top;
        self
    }

    /// Sets the border size for the bottom edge of the 9-slice scaling.
    ///
    /// This method will override only the bottom edge of the border.
    pub fn bottom_border(mut self, bottom: f32) -> Self {
        self.slicer.border.bottom = bottom;
        self
    }

    /// Sets the border size for the left edge of the 9-slice scaling.
    ///
    /// This method will override only the left edge of the border.
    pub fn left_border(mut self, left: f32) -> Self {
        self.slicer.border.left = left;
        self
    }

    /// Sets the border size for the right edge of the 9-slice scaling.
    ///
    /// This method will override only the right edge of the border.
    pub fn right_border(mut self, right: f32) -> Self {
        self.slicer.border.right = right;
        self
    }

    /// Sets the center of the image texture to tile to fill the area instead of
    /// stretching.
    ///
    /// The stretch value defines how much the center of the image is allowed to
    /// stretch before being repeated.
    ///
    /// If this value is not set, the center of the image will be stretched to
    /// fill the area.
    pub fn tile_center(mut self, stretch_value: f32) -> Self {
        self.slicer.center_scale_mode = SliceScaleMode::Tile { stretch_value };
        self
    }

    /// Sets the edges of the image texture to tile to fill the area instead of
    /// stretching.
    ///
    /// The stretch value defines how much the center of the image is allowed to
    /// stretch before being repeated.
    ///
    /// If this value is not set, the edges of the image will be stretched to
    /// fill the area.
    pub fn tile_sides(mut self, stretch_value: f32) -> Self {
        self.slicer.sides_scale_mode = SliceScaleMode::Tile { stretch_value };
        self
    }

    /// Sets how much the corners of the image texture are allowed to scale up,
    /// relative to the original texture size. This value defaults to 1.0.
    pub fn max_corner_scale(mut self, max_corner_scale: f32) -> Self {
        self.slicer.max_corner_scale = max_corner_scale;
        self
    }
}

impl From<TexSlicedBuilder> for NodeTextureScaling {
    fn from(value: TexSlicedBuilder) -> Self {
        NodeTextureScaling::Sliced {
            slicer: value.slicer,
        }
    }
}
