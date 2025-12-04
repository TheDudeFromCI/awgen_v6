//! This module implements a widget that previews images in a grid layout. This
//! can be used for thing such as a file explorer or asset explorer.

use bevy::prelude::*;

use crate::prelude::InteractionSender;
use crate::theme::UiTheme;

/// A builder for a grid cell node. This can be used when initializing a
/// [`GridPreview`] with a set of initial cells.
#[derive(Debug, Clone)]
pub struct GridNodeBuilder {
    /// The image to display in the grid cell.
    pub icon: Handle<Image>,

    /// The label to display below the image.
    pub label: String,
}

/// A widget that displays a grid preview of images. Useful for asset explorers.
#[derive(Debug, Component)]
#[require(Node)]
pub struct GridPreview {
    /// The theme for the grid preview.
    theme: UiTheme,

    /// The ID of the panel that items are added to.
    ///
    /// This value is assigned when the preview is initialized.
    panel_id: Option<Entity>,

    /// An optional list of initial cells to populate the grid with. This value
    /// will be discarded after the grid is initialized.
    init_cells: Option<Vec<GridNodeBuilder>>,
}

impl GridPreview {
    /// Creates a new grid preview with the given cell size and padding.
    pub fn new(theme: UiTheme) -> Self {
        Self {
            theme,
            panel_id: None,
            init_cells: None,
        }
    }

    /// Creates a new grid preview with the given cell size, padding, and
    /// initial cells.
    pub fn with_cells(theme: UiTheme, cells: Vec<GridNodeBuilder>) -> Self {
        Self {
            theme,
            panel_id: None,
            init_cells: Some(cells),
        }
    }
}

/// Observer system that runs when a [`GridPreview`] component is added.
pub(crate) fn on_grid_add(
    trigger: On<Add, GridPreview>,
    mut query: Query<(&mut Node, &mut GridPreview)>,
    mut commands: Commands,
) {
    let Ok((mut node, mut grid)) = query.get_mut(trigger.entity) else {
        error!("GridPreview added to entity without Node component");
        return;
    };

    node.flex_direction = FlexDirection::Column;

    let panel_id = commands
        .spawn((
            ChildOf(trigger.entity),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                row_gap: px(grid.theme.grid_preview.cell_spacing.y),
                column_gap: px(grid.theme.grid_preview.cell_spacing.x),
                overflow: Overflow::scroll_y(),
                scrollbar_width: 4.0,
                width: percent(100.0),
                ..default()
            },
        ))
        .id();
    grid.panel_id = Some(panel_id);

    commands
        .entity(trigger.entity)
        .insert(grid.theme.inner_window.clone());

    if let Some(cells) = grid.init_cells.take() {
        for cell in cells {
            commands.spawn((
                ChildOf(panel_id),
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: px(4.0),
                    ..default()
                },
                grid.theme.grid_preview.cell.clone(),
                InteractionSender,
                children![
                    (
                        Node {
                            width: px(grid.theme.grid_preview.cell_size.x),
                            height: px(grid.theme.grid_preview.cell_size.y),
                            ..default()
                        },
                        ImageNode {
                            image: cell.icon,
                            ..default()
                        },
                        BorderRadius::all(px(grid.theme.grid_preview.cell.border_radius)),
                    ),
                    (
                        Text::from(cell.label),
                        grid.theme.grid_preview.cell.text.clone()
                    )
                ],
            ));
        }
    }
}
