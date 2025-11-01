//! This module implements the components use in the tree view widget plugin.

use bevy::prelude::*;

use crate::style::Style;

/// A tree view component.
#[derive(Debug, Component)]
#[require(Node, Style)]
pub struct TreeView {}

pub struct TreeViewElement {}
