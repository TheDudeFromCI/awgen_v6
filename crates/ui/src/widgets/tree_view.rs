//! This module implements the tree view widget.

use bevy::ecs::relationship::RelatedSpawner;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::color::InteractiveColor;
use crate::prelude::InteractionSender;
use crate::theme::UiTheme;

/// A builder for a single tree node within a [`TreeView`].
///
/// This can be used to construct a tree view with a specific hierarchy when
/// initializing a new [`TreeView`].
#[derive(Debug, Default, Clone)]
pub struct TreeNodeBuilder {
    /// The content of the tree node.
    pub content: TreeNodeContent,

    /// The children of the tree node.
    pub children: Vec<TreeNodeBuilder>,
}

/// A [`TreeView`] component.
#[derive(Debug, Component)]
#[require(Node)]
pub struct TreeView {
    /// The root node of the tree view.
    ///
    /// This node will not be visible; it is only used to store the root
    /// elements.
    root_node: Option<Entity>,

    /// The theme for the tree view. This will be cloned for each node.
    theme: UiTheme,

    /// An optional builder used to initialize the tree view. This is only used
    /// when the tree view is first created and then discarded.
    builder: Option<TreeNodeBuilder>,
}

impl TreeView {
    /// Creates a new tree view with the given theme.
    pub fn new(theme: UiTheme) -> Self {
        Self {
            root_node: None,
            theme,
            builder: None,
        }
    }

    /// Sets the builder for the tree view. This builder will be used to
    /// initialize the tree view when it is first created.
    ///
    /// The builder represents the root node of the tree. It's content will not
    /// be displayed, but its children will be added as the top-level nodes of
    /// the tree view.
    pub fn from_builder(theme: UiTheme, builder: TreeNodeBuilder) -> Self {
        Self {
            root_node: None,
            theme,
            builder: Some(builder),
        }
    }

    /// Gets a reference to the theme of the tree view.
    pub fn theme(&self) -> &UiTheme {
        &self.theme
    }

    /// Gets the root node of the tree view. The root node is a special hidden
    /// node that contains the top-level elements of the tree.
    ///
    /// If the tree view has not been initialized yet, this will return `None`.
    pub fn root_node(&self) -> Option<Entity> {
        self.root_node
    }
}

/// A single node within a tree view.
#[derive(Debug, Component)]
#[require(Node)]
pub struct TreeNode {
    /// The depth of the tree node.
    depth: u16,

    /// The tree view this node belongs to.
    tree: Entity,
}

impl TreeNode {
    /// Gets the depth of the tree node.
    pub fn depth(&self) -> u16 {
        self.depth
    }
}

/// The content of a tree node.
#[derive(Debug, Default, Clone)]
pub struct TreeNodeContent {
    /// The text of the tree node.
    pub text: String,

    /// An optional icon for the tree node.
    pub icon: Option<Handle<Image>>,
}

impl<S> From<S> for TreeNodeContent
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self {
            text: value.into(),
            icon: None,
        }
    }
}

/// A SystemParam for editing tree views and their nodes.
#[derive(SystemParam)]
pub struct TreeEditor<'w, 's> {
    /// The tree views in the world.
    trees: Query<'w, 's, &'static TreeView>,

    /// The tree nodes in the world.
    tree_nodes: Query<'w, 's, &'static TreeNode>,

    /// The commands to modify the world.
    commands: Commands<'w, 's>,
}

impl<'w, 's> TreeEditor<'w, 's> {
    /// Begins editing the specified tree view, returning a [`TreeNodeEditor`]
    /// for the root node.
    ///
    /// Returns an error if the specified `tree` is not found or has not been
    /// initialized.
    pub fn tree(&mut self, tree: Entity) -> Result<TreeNodeEditor<'_>, TreeEditorError> {
        let tree_node = self
            .trees
            .get(tree)
            .map_err(|_| TreeEditorError::TreeNotFound(tree))?;

        let root_node = tree_node
            .root_node
            .ok_or(TreeEditorError::TreeNotInitialized(tree))?;

        Ok(TreeNodeEditor {
            commands: self.commands.reborrow(),
            tree,
            node: root_node,
            theme: tree_node.theme.clone(),
            depth: 0,
        })
    }

    /// Begins editing the specified tree node, returning a [`TreeNodeEditor`]
    /// for that node.
    pub fn node(&mut self, node: Entity) -> Result<TreeNodeEditor<'_>, TreeEditorError> {
        let tree_node = self
            .tree_nodes
            .get(node)
            .map_err(|_| TreeEditorError::TreeNodeNotFound(node))?;

        let tree_view = self
            .trees
            .get(tree_node.tree)
            .map_err(|_| TreeEditorError::TreeNotFound(tree_node.tree))?;

        Ok(TreeNodeEditor {
            commands: self.commands.reborrow(),
            tree: tree_node.tree,
            node,
            theme: tree_view.theme.clone(),
            depth: tree_node.depth,
        })
    }
}

/// An editor for a specific tree node within a tree view.
pub struct TreeNodeEditor<'a> {
    /// The commands to modify the world.
    commands: Commands<'a, 'a>,

    /// The tree view this node belongs to.
    tree: Entity,

    /// The theme for the tree view.
    theme: UiTheme,

    /// The current node being edited.
    node: Entity,

    /// The depth of the current node.
    depth: u16,
}

impl<'a> TreeNodeEditor<'a> {
    /// Adds a new node as a child of the specified parent node, and returns the
    /// ID of the newly created node.
    ///
    /// The specified `node` must be a valid [`TreeNode`] entity.
    pub fn add_child(mut self, content: TreeNodeContent) -> TreeNodeEditor<'a> {
        let id = self
            .commands
            .spawn(build_node(
                self.node,
                self.tree,
                content.clone(),
                self.depth + 1,
                &self.theme,
                false,
                false,
            ))
            .id();

        self.depth += 1;
        self.node = id;
        self
    }

    /// Removes the current node from the tree.
    ///
    /// If the node currently being edited is the root node, all its children
    /// will be removed instead.
    pub fn remove(mut self) {
        if self.depth == 0 {
            self.commands.entity(self.node).despawn_children();
        } else {
            self.commands.entity(self.node).despawn();
        }
    }
}

/// Errors that can occur when editing a tree view.
#[derive(Debug, thiserror::Error)]
pub enum TreeEditorError {
    /// The specified tree view was not found.
    #[error("Tree view not found: {0}")]
    TreeNotFound(Entity),

    /// The specified tree view has not been initialized.
    #[error("Tree view not initialized: {0}")]
    TreeNotInitialized(Entity),

    /// The specified tree node was not found.
    #[error("Tree node not found: {0}")]
    TreeNodeNotFound(Entity),
}

/// When a [`TreeView`] is added, set up its node properties.
pub(crate) fn on_tree_added(
    trigger: On<Add, TreeView>,
    mut query: Query<(&mut Node, &mut TreeView)>,
    mut commands: Commands,
) {
    let Ok((mut node, mut tree)) = query.get_mut(trigger.entity) else {
        error!("Failed to query tree view node");
        return;
    };

    node.display = Display::Flex;
    node.flex_direction = FlexDirection::Column;
    node.overflow = Overflow::scroll();
    // node.scrollbar_width = 4.0;
    node.scrollbar_width = 0.0;

    commands
        .entity(trigger.entity)
        .insert(tree.theme.tree_view.container.clone());

    let builder = tree.builder.take().unwrap_or_default();
    tree.root_node = Some(build_tree_recursive(
        &mut commands,
        trigger.entity,
        trigger.entity,
        builder,
        0,
        &tree.theme,
    ));
}

/// Recursively builds the tree nodes from the given builder.
fn build_tree_recursive(
    commands: &mut Commands,
    tree: Entity,
    parent: Entity,
    builder: TreeNodeBuilder,
    depth: u16,
    theme: &UiTheme,
) -> Entity {
    let id = if depth == 0 {
        commands
            .spawn((
                ChildOf(parent),
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                TreeNode { depth, tree },
            ))
            .id()
    } else {
        commands
            .spawn(build_node(
                parent,
                tree,
                builder.content,
                depth,
                theme,
                !builder.children.is_empty(),
                false,
            ))
            .id()
    };

    for child_builder in builder.children {
        build_tree_recursive(commands, tree, id, child_builder, depth + 1, theme);
    }

    id
}

/// Builds a single tree node bundle.
fn build_node(
    parent: Entity,
    tree: Entity,
    content: TreeNodeContent,
    depth: u16,
    theme: &UiTheme,
    has_children: bool,
    is_collapsed: bool,
) -> impl Bundle {
    let right_arrow_icon = theme.tree_view.right_arrow_icon.clone();
    let down_arrow_icon = theme.tree_view.down_arrow_icon.clone();
    let spacer_icon = theme.tree_view.spacer_icon.clone();
    let icon_size = theme.tree_view.container.icon_size;
    let label_theme = theme.tree_view.label.clone();

    (
        ChildOf(parent),
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        TreeNode { depth, tree },
        children![(
            Node {
                flex_direction: FlexDirection::Row,
                ..default()
            },
            theme.tree_view.label.clone(),
            InteractionSender,
            Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
                for _ in 1 .. depth {
                    parent.spawn((
                        Node {
                            width: px(icon_size),
                            height: px(icon_size),
                            ..default()
                        },
                        ImageNode {
                            image: spacer_icon.clone(),
                            ..default()
                        },
                        InteractiveColor::<ImageNode>::from(&label_theme.icon_color),
                    ));
                }

                parent.spawn((
                    Node {
                        width: px(icon_size),
                        height: px(icon_size),
                        ..default()
                    },
                    ImageNode {
                        image: match (has_children, is_collapsed) {
                            (true, false) => right_arrow_icon,
                            (true, true) => down_arrow_icon,
                            (false, _) => spacer_icon.clone(),
                        },
                        ..default()
                    },
                    InteractiveColor::<ImageNode>::from(&label_theme.icon_color),
                ));

                if let Some(icon) = content.icon {
                    parent.spawn((
                        Node {
                            width: px(icon_size),
                            height: px(icon_size),
                            ..default()
                        },
                        ImageNode {
                            image: icon,
                            ..default()
                        },
                        InteractiveColor::<ImageNode>::from(&label_theme.icon_color),
                    ));
                }

                parent.spawn((Text::from(content.text), label_theme.text.clone()));
            })),
        ),],
    )
}
