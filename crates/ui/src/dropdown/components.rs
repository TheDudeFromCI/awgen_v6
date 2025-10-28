//! This module implements various components used by the dropdown widget.

use bevy::prelude::*;

use crate::style::Style;

/// A dropdown menu component.
#[derive(Debug, Component)]
#[require(Node, DropdownMenuState, Style)]
pub struct DropdownMenu {
    /// The entries in the dropdown menu.
    ///
    /// This field is only used to initialize the menu; it is not updated after
    /// creation.
    entries: Vec<DropdownMenuEntry>,

    /// The main entry of the dropdown menu.
    main: DropdownMenuEntry,
}

impl DropdownMenu {
    /// Creates a new [`DropdownMenu`] with the given entries.
    pub fn new(main: DropdownMenuEntry, entries: Vec<DropdownMenuEntry>) -> Self {
        Self { main, entries }
    }

    /// Returns the main button of the dropdown menu. This is the button that
    /// toggles the menu open and closed.
    pub fn main_button(&self) -> &DropdownMenuEntry {
        &self.main
    }

    /// Returns the entries in the dropdown menu.
    pub fn entries(&self) -> &[DropdownMenuEntry] {
        &self.entries
    }
}

/// A component that tracks the open/closed state of a dropdown menu.
#[derive(Debug, Default, Component, Deref, DerefMut)]
pub struct DropdownMenuState(bool);

impl DropdownMenuState {
    /// Returns `true` if the dropdown menu is open.
    pub fn is_open(&self) -> bool {
        self.0
    }

    /// Sets the open/closed state of the dropdown menu.
    pub fn set_open(&mut self, open: bool) {
        self.0 = open;
    }
}

/// An entry in a dropdown menu.
#[derive(Debug)]
pub struct DropdownMenuEntry {
    /// The icon of the entry, if any.
    pub icon: Option<Handle<Image>>,

    /// The text of the entry, if any.
    pub text: Option<String>,
}

/// A component that marks an entity as a dropdown menu button.
#[derive(Debug, Component)]
#[require(bevy::ui_widgets::Button)]
pub struct DropdownMenuButton {
    /// The menu root entity this button belongs to.
    pub menu: Entity,
}

/// A component that marks an entity as a dropdown entry button.
#[derive(Debug, Component)]
#[require(bevy::ui_widgets::Button)]
pub struct DropdownEntryButton {
    /// The menu root entity this entry belongs to.
    pub menu: Entity,
}

/// A component that holds the important entities of a dropdown menu.
#[derive(Debug, Component)]
pub struct DropdownMenuNodes {
    /// The content node of the dropdown menu.
    pub(super) content_node: Entity,
}
