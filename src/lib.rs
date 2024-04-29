//! This crate provides a full-featured UI system for Bevy to make creating
//! and managing UIs easier.
//!
//! The goal of this crate is to create a more intuitive API for creating a user
//! interface with less boilerplate code.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;
use nodes::text_field::CURSOR_HANDLE;

pub mod blocks;
pub mod builders;
pub mod nodes;

#[doc(hidden)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{blocks::*, builders::*, nodes::*, StreamlineUIPlugin};
}

/// This plugin provides a full-featured UI system for Bevy to make creating
/// and managing UIs easier.
///
/// The goal of this plugin is to allow for a user interface, including menus,
/// buttons, GUIs, popups, etc, to be created and managed with minimal effort.
pub struct StreamlineUIPlugin;
impl Plugin for StreamlineUIPlugin {
    fn build(&self, _app: &mut App) {
        load_internal_binary_asset!(
            _app,
            CURSOR_HANDLE,
            "../assets/fonts/Cursor.ttf",
            |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
        );

        _app.add_systems(Update, nodes::text_field::handle_text_input);
    }
}
