//! This module implements the tree view widget.

use bevy::ecs::relationship::RelatedSpawner;
use bevy::prelude::*;

use crate::theme::UiTheme;
use crate::widgets::button::{ButtonBuilder, ButtonContent, button};

/// Builder for a tree view widget.
#[derive(Debug, Clone)]
pub struct TreeBuilder {
    /// The default node component, if a custom layout is needed. Some fields
    /// may be overridden.
    pub node: Node,

    /// The root elements of the tree.
    pub root_elements: Vec<TreeNode>,

    /// The theme for the tree view.
    pub theme: UiTheme,
}

/// A node in the tree view.
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// The content of the tree node.
    pub content: ButtonContent,

    /// The children of the tree node.
    pub children: Vec<TreeNode>,
}

/// Creates a tree view UI component using the provided builder.
pub fn tree_view(builder: TreeBuilder) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            border: UiRect::all(px(builder.theme.inner_window.border_thickness)),
            padding: UiRect::all(px(builder.theme.inner_window.padding)),
            overflow: Overflow::scroll(),
            scrollbar_width: 4.0,
            ..builder.node
        },
        BackgroundColor(builder.theme.inner_window.background_color),
        BorderColor::all(builder.theme.inner_window.border_color),
        BorderRadius::all(px(builder.theme.inner_window.border_radius)),
        ScrollPosition::default(),
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            for node in builder.root_elements {
                tree_node_recursive(node, parent, &builder.theme, 0.0);
            }
        })),
    )
}

/// Recursively creates tree nodes.
fn tree_node_recursive(
    node: TreeNode,
    parent: &mut RelatedSpawner<ChildOf>,
    theme: &UiTheme,
    indent: f32,
) {
    parent.spawn(button(ButtonBuilder {
        node: Node {
            left: px(indent),
            ..default()
        },
        content: node.content,
        theme: theme.clone(),
    }));

    for node in node.children {
        tree_node_recursive(node, parent, theme, indent + 20.0);
    }
}
