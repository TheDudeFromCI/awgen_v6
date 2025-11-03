//! This module implements the components use in the tree view widget plugin.

use bevy::prelude::*;

use crate::button::ButtonContent;

/// An element in the tree view.
#[derive(Debug, Clone)]
pub enum TreeViewElement {
    /// A folder that can contain other elements.
    Folder(Vec<TreeViewElement>),

    /// A file represented by a button.
    File(Box<ButtonContent>),
}
