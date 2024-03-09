//! Defines a builder for creating a node bundle to be generated using entity
//! commands.

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::prelude::UiNode;

/// A consumable function that adds a component bundle to an entity.
type ComponentWriter = Box<dyn FnOnce(&mut EntityCommands)>;

/// An enum that represents the type of node bundle to create.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum NodeBundleType {
    /// The default node bundle type.
    #[default]
    Node,

    /// A node bundle that contains an image.
    Image,

    /// A node bundle that contains a button.
    Button,

    /// A node bundle that contains text.
    Text,
}

impl NodeBundleType {
    /// Consumes this node bundle type and creates a new entity with the
    /// appropriate node bundle.
    fn into_bundle<'a>(self, cmd: &'a mut Commands) -> EntityCommands<'a> {
        match self {
            NodeBundleType::Node => cmd.spawn(NodeBundle::default()),
            NodeBundleType::Image => cmd.spawn(ImageBundle::default()),
            NodeBundleType::Button => cmd.spawn(ButtonBundle::default()),
            NodeBundleType::Text => cmd.spawn(TextBundle::default()),
        }
    }
}

/// This builder can be used to define how a node bundle should be created when
/// generating a new entity.
///
/// The purpose of this struct is to deffer the creation of a node bundle until
/// until all blocks have been processes, allowing each block to modify this
/// builder one at a time. After the final state of the builder has been
/// determined, the builder can be consumed to create a node bundle using entity
/// commands.
#[derive(Default)]
pub struct NodeBundleBuilder {
    /// The parent entity of the node, if any.
    parent: Option<Entity>,

    /// The style of the node.
    ///
    /// This component is separate from the others due to it's complexity. It
    /// contains many fields and is often modified by many different blocks.
    /// Other components, such as background color, as safe to replace per
    /// block.
    style: Style,

    /// The type of node bundle to create.
    bundle: NodeBundleType,

    /// A list of component bundles to apply to the entity after creation. All
    /// component bundles within this list will be applied in the order they
    /// were added, and will replace previous components if they exist.
    components: Vec<ComponentWriter>,

    /// The children of the node.
    children: Vec<UiNode>,
}

impl NodeBundleBuilder {
    /// Gets a mutable reference to the style component of the node.
    ///
    /// This method is available for blocks that need to modify the style of the
    /// node, but only wish to modify certain fields rather than replace the
    /// entire style component.
    pub fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    /// Sets the type of node bundle to use when creating the entity.
    pub fn bundle_type(&mut self, bundle: NodeBundleType) {
        self.bundle = bundle;
    }

    /// Adds a component bundle to the list of component bundles to apply to the
    /// entity after creation.
    ///
    /// Component bundles will be applied in the order they were added, and will
    /// replace previous components if they exist.
    pub fn insert(&mut self, components: impl Bundle) {
        self.components
            .push(Box::new(move |entity: &mut EntityCommands| {
                entity.insert(components);
            }));
    }

    /// Sets the parent entity of the node.
    ///
    /// If the parent entity is `None`, the node will be a root node.
    pub fn set_parent(&mut self, parent: Option<Entity>) {
        self.parent = parent;
    }

    /// Sets the children of the node.
    pub fn set_children(&mut self, children: Vec<UiNode>) {
        self.children = children;
    }

    /// Consumes this builder to spawn a new entity with the defined node
    /// bundle.
    ///
    /// Returns the entity that was spawned.
    pub fn build(self, cmd: &mut Commands, asset_server: &AssetServer) -> Entity {
        // This method relies on the fact that inserting new components into an
        // entity will replace any existing components of the same type.

        // TODO: Look into creating a unified component bundle before spawning
        // the entity, rather than spawning the entity and then adding each
        // component one at a time.

        let mut entity_cmd = self.bundle.into_bundle(cmd);
        let id = entity_cmd.id();
        entity_cmd.insert(self.style);

        if let Some(parent_entity) = self.parent {
            entity_cmd.set_parent(parent_entity);
        }

        for component in self.components {
            component(&mut entity_cmd);
        }

        for child in self.children {
            child.build_node(cmd, asset_server, Some(id));
        }

        id
    }
}
