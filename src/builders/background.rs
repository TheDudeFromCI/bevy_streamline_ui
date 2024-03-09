//! A builder for defining the background of a UI node.

use bevy::prelude::*;

use crate::prelude::{NodeBackground, NodeTextureScaling};

/// A builder for defining the background of a UI node.
#[derive(Debug, Default, Clone)]
pub struct BackgroundBuilder;

impl BackgroundBuilder {
    /// Sets the node to have no background.
    pub fn none() -> EmptyBackgroundBuilder {
        EmptyBackgroundBuilder
    }

    /// Sets the node to have a solid color background.
    pub fn color(color: Color) -> ColorBackgroundBuilder {
        ColorBackgroundBuilder { color }
    }

    /// Sets the node to have a background image.
    pub fn image<T>(img: T) -> ImageBackgroundBuilder
    where
        T: Into<String>,
    {
        ImageBackgroundBuilder {
            img: img.into(),
            ..default()
        }
    }
}

/// A builder for defining the background of a UI node with no background.
#[derive(Debug, Default, Clone)]
pub struct EmptyBackgroundBuilder;

impl From<EmptyBackgroundBuilder> for NodeBackground {
    fn from(_: EmptyBackgroundBuilder) -> Self {
        NodeBackground::None
    }
}

/// A builder for defining the background of a UI node with a solid color
/// background.
#[derive(Debug, Default, Clone)]
pub struct ColorBackgroundBuilder {
    /// The color of the background.
    color: Color,
}

impl From<ColorBackgroundBuilder> for NodeBackground {
    fn from(builder: ColorBackgroundBuilder) -> Self {
        NodeBackground::Color {
            color: builder.color,
        }
    }
}

/// A builder for defining the background of a UI node with a background image.
#[derive(Debug, Default, Clone)]
pub struct ImageBackgroundBuilder {
    /// The background image to use for the node.
    img: String,

    /// The tint color to use for the image.
    tint: Color,

    /// The texture scaling mode to use for the background image.
    tex_scaling: NodeTextureScaling,
}

impl ImageBackgroundBuilder {
    /// Sets the background image to use for the node.
    pub fn image<T>(mut self, img: T) -> Self
    where
        T: Into<String>,
    {
        self.img = img.into();
        self
    }

    /// Sets the tint color to use for the image.
    pub fn tint(mut self, tint: Color) -> Self {
        self.tint = tint;
        self
    }

    /// Sets the texture scaling mode to use for the background image.
    pub fn texture_scaling<T>(mut self, scaling: T) -> Self
    where
        T: Into<NodeTextureScaling>,
    {
        self.tex_scaling = scaling.into();
        self
    }
}

impl From<ImageBackgroundBuilder> for NodeBackground {
    fn from(builder: ImageBackgroundBuilder) -> Self {
        NodeBackground::Image {
            img: builder.img,
            tint: builder.tint,
            tex_scaling: builder.tex_scaling,
        }
    }
}
