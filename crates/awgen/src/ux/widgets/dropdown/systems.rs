//! This module implements the systems for the dropdown widget.

use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use crate::ux::widgets::dropdown::ui::build_menu;
use crate::ux::widgets::dropdown::{
    DropdownEntryButton,
    DropdownMenu,
    DropdownMenuButton,
    DropdownMenuNodes,
    DropdownMenuState,
};

/// An observer system that runs when a [`DropdownMenu`] is spawned.
pub(super) fn on_spawn(
    trigger: On<Add, DropdownMenu>,
    asset_server: Res<AssetServer>,
    menu: Query<&DropdownMenu>,
    mut commands: Commands,
) {
    let entity = trigger.entity;
    let menu = menu.get(entity).unwrap();
    debug!("Spawning DropdownMenu: {}", entity);

    build_menu(&asset_server, entity, menu, &mut commands);
}

/// An observer system that runs when a [`DropdownMenuButton`] is clicked.
pub(super) fn on_menu_click(
    trigger: On<Activate>,
    buttons: Query<&DropdownMenuButton>,
    mut menu_states: Query<&mut DropdownMenuState>,
) {
    let Ok(menu_button) = buttons.get(trigger.entity) else {
        return;
    };

    let Ok(mut menu_state) = menu_states.get_mut(menu_button.menu) else {
        error!("DropdownMenu entity not found");
        return;
    };

    let open = !menu_state.is_open();
    menu_state.set_open(open);

    debug!(
        "Toggled DropdownMenu {}; set to open: {}",
        menu_button.menu, open
    );
}

/// An observer system that runs when a [`DropdownEntryButton`] is clicked.
pub(super) fn on_menu_entry_click(
    trigger: On<Activate>,
    buttons: Query<&DropdownEntryButton>,
    mut menu_states: Query<&mut DropdownMenuState>,
) {
    let Ok(entry_button) = buttons.get(trigger.entity) else {
        return;
    };

    let Ok(mut menu_state) = menu_states.get_mut(entry_button.menu) else {
        error!("DropdownMenu entity not found");
        return;
    };

    menu_state.set_open(false);

    debug!(
        "Closed DropdownMenu {} due to entry click",
        entry_button.menu
    );
}

/// A system that toggles the visibility of dropdown menu entries based on the
/// open/closed state of the menu.
pub(super) fn update_menu_visibility(
    menus: Query<(Entity, &DropdownMenuNodes, &DropdownMenuState), Changed<DropdownMenuState>>,
    mut nodes: Query<&mut Node>,
) {
    for (menu_id, menu_nodes, menu_state) in menus.iter() {
        debug!(
            "Updating visibility for DropdownMenu {} entries; is_open: {}",
            menu_id,
            menu_state.is_open()
        );

        let Ok(mut node) = nodes.get_mut(menu_nodes.content_node) else {
            error!("Failed to update menu visibility, DropdownEntryButton node not found");
            continue;
        };

        node.display = if menu_state.is_open() {
            Display::Flex
        } else {
            Display::None
        };
    }
}
