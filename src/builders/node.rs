//! A builder for defining a [`UiNode`].

use crate::prelude::{NodeBackground, NodePosition, NodeText, NodeTextField, UiNode};

/// A builder for defining a [`UiNode`].
#[derive(Debug, Default, Clone)]
pub struct UiNodeBuilder;

impl UiNodeBuilder {
    /// Sets the type of the node to be a canvas.
    pub fn canvas() -> CanvasNodeBuilder {
        CanvasNodeBuilder::default()
    }

    /// Sets the type of the node to be a panel.
    pub fn panel() -> PanelNodeBuilder {
        PanelNodeBuilder::default()
    }

    /// Sets the type of the node to be a text.
    pub fn text<T: Into<NodeText>>(text: T) -> TextNodeBuilder {
        TextNodeBuilder {
            text: text.into(),
            ..Default::default()
        }
    }

    /// Sets the type of the node to be a text field.
    pub fn text_field<T: Into<NodeTextField>>(field: T) -> TextFieldNodeBuilder {
        TextFieldNodeBuilder {
            text_field: field.into(),
            ..Default::default()
        }
    }
}

/// A builder for defining a canvas node.
#[derive(Debug, Default, Clone)]
pub struct CanvasNodeBuilder {
    /// The children of the canvas.
    children: Vec<UiNode>,
}

impl CanvasNodeBuilder {
    /// Adds a child to the canvas.
    pub fn child<T: Into<UiNode>>(mut self, child: T) -> Self {
        self.children.push(child.into());
        self
    }
}

impl From<CanvasNodeBuilder> for UiNode {
    fn from(builder: CanvasNodeBuilder) -> Self {
        UiNode::Canvas {
            children: builder.children.into(),
        }
    }
}

/// A builder for defining a panel node.
#[derive(Debug, Default, Clone)]
pub struct PanelNodeBuilder {
    /// The background of the panel.
    background: Option<NodeBackground>,

    /// The position of the panel.
    position: Option<NodePosition>,

    /// The children of the panel.
    children: Vec<UiNode>,
}

impl PanelNodeBuilder {
    /// Sets the background of the panel.
    pub fn background<T: Into<NodeBackground>>(mut self, background: T) -> Self {
        self.background = Some(background.into());
        self
    }

    /// Sets the position of the panel.
    pub fn position<T: Into<NodePosition>>(mut self, position: T) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Adds a child to the panel.
    pub fn child<T: Into<UiNode>>(mut self, child: T) -> Self {
        self.children.push(child.into());
        self
    }
}

impl From<PanelNodeBuilder> for UiNode {
    fn from(builder: PanelNodeBuilder) -> Self {
        UiNode::Panel {
            background: builder.background.unwrap_or_default(),
            position: builder.position.unwrap_or_default(),
            children: builder.children.into(),
        }
    }
}

/// A builder for defining a text node.
#[derive(Debug, Default, Clone)]
pub struct TextNodeBuilder {
    /// The background of the text.
    background: NodeBackground,

    /// The position of the text.
    position: NodePosition,

    /// The text data for the text.
    text: NodeText,
}

impl TextNodeBuilder {
    /// Sets the background of the text.
    pub fn background<T: Into<NodeBackground>>(mut self, background: T) -> Self {
        self.background = background.into();
        self
    }

    /// Sets the position of the text.
    pub fn position<T: Into<NodePosition>>(mut self, position: T) -> Self {
        self.position = position.into();
        self
    }

    /// Sets the text data for the text.
    pub fn text<T: Into<NodeText>>(mut self, text: T) -> Self {
        self.text = text.into();
        self
    }
}

impl From<TextNodeBuilder> for UiNode {
    fn from(builder: TextNodeBuilder) -> Self {
        UiNode::Text {
            background: builder.background,
            position: builder.position,
            text: builder.text,
        }
    }
}

/// A builder for defining a text field node.
#[derive(Debug, Default, Clone)]
pub struct TextFieldNodeBuilder {
    /// The background of the text field.
    background: NodeBackground,

    /// The position of the text field.
    position: NodePosition,

    /// The text field data for the text field.
    text_field: NodeTextField,
}

impl TextFieldNodeBuilder {
    /// Sets the background of the text field.
    pub fn background<T: Into<NodeBackground>>(mut self, background: T) -> Self {
        self.background = background.into();
        self
    }

    /// Sets the position of the text field.
    pub fn position<T: Into<NodePosition>>(mut self, position: T) -> Self {
        self.position = position.into();
        self
    }

    /// Sets the text field data for the text field.
    pub fn text_field<T: Into<NodeTextField>>(mut self, text_field: T) -> Self {
        self.text_field = text_field.into();
        self
    }
}

impl From<TextFieldNodeBuilder> for UiNode {
    fn from(builder: TextFieldNodeBuilder) -> Self {
        UiNode::TextField {
            background: builder.background,
            position: builder.position,
            text_field: builder.text_field,
        }
    }
}
