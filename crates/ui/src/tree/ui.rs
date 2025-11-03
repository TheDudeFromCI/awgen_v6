//! This module implements the UI for a tree view widget.

use bevy::prelude::*;

use crate::style::Style;
use crate::tree::TreeViewElement;

/// Create a bundle for the tree view widget.
///
/// The provided [`Node`] component will be used for the base entity, and
/// modified to layout child elements.
pub fn awgen_tree_view(node: Node, tree: TreeViewElement, style: &Style) -> impl Bundle {}
