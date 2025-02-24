//! This module implements the Editor UI plugin, which provides the base user
//! interface for the editor using Egui.

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::file_picker::{FilePickerDialog, FileSelectionResult};
use crate::project::ProjectSettings;

/// This plugin provides the base user interface for the editor.
pub struct EditorUiPlugin;
impl Plugin for EditorUiPlugin {
    fn build(&self, app_: &mut App) {
        app_.init_resource::<OccupiedScreenSpace>()
            .add_systems(Update, layout);
    }
}

/// This resource stores the screen space occupied by the given UI panels. These
/// values are updated every frame by the `layout` system.
#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    /// The space occupied on the left side of the screen.
    pub left: f32,

    /// The space occupied on the top side of the screen.
    pub top: f32,

    /// The space occupied on the right side of the screen.
    pub right: f32,

    /// The space occupied on the bottom side of the screen.
    pub bottom: f32,
}

/// The operation to perform when selecting a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FileSelectionOperation {
    /// Create a new project at the indicated path.
    NewProject,

    /// Open the project at the indicated path.
    OpenProject,
}

/// Render the editor UI layout using Egui.
fn layout(
    mut file_picker: Local<Option<(FilePickerDialog, FileSelectionOperation)>>,
    mut app_exit: EventWriter<AppExit>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut contexts: EguiContexts,
    project_settings: Option<Res<ProjectSettings>>,
) {
    let ctx = contexts.ctx_mut();

    let mut is_picking_file = false;
    let is_project_open = project_settings.is_some();

    if let Some((file_dialog, file_op)) = file_picker.as_mut() {
        if file_dialog.is_open() {
            is_picking_file = true;
        }

        if let Some(result) = file_dialog.poll() {
            match file_op {
                FileSelectionOperation::NewProject => match result {
                    FileSelectionResult::Path(path) => {
                        info!("Creating new project at: {}", path);
                    }
                    FileSelectionResult::Canceled => {
                        debug!("New project creation canceled.");
                    }
                },
                FileSelectionOperation::OpenProject => match result {
                    FileSelectionResult::Path(path) => {
                        info!("Opening project at: {}", path);
                    }
                    FileSelectionResult::Canceled => {
                        debug!("Project open canceled.");
                    }
                },
            }

            *file_picker = None;
        }
    }

    occupied_screen_space.left = egui::TopBottomPanel::top("toolbar")
        .resizable(false)
        .show(ctx, |ui| {
            if is_picking_file {
                ui.disable();
            }

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.add_enabled_ui(!is_project_open, |ui| {
                        if ui.button("New Project").clicked() {
                            info!("Creating new project.");
                            ui.close_menu();

                            *file_picker = Some((
                                FilePickerDialog::select_folder("Select project path".to_string()),
                                FileSelectionOperation::NewProject,
                            ));
                        }

                        if ui.button("Open Project").clicked() {
                            debug!("Opening project file.");
                            ui.close_menu();

                            *file_picker = Some((
                                FilePickerDialog::select_file(
                                    "Open project file".to_string(),
                                    vec!["*.awgen".to_string()],
                                    "Awgen project file".to_string(),
                                ),
                                FileSelectionOperation::OpenProject,
                            ));
                        }
                    });

                    ui.add_enabled_ui(is_project_open, |ui| {
                        if ui.button("Close Project").clicked() {
                            debug!("Closing project file.");
                            ui.close_menu();
                        }
                    });

                    ui.separator();

                    ui.menu_button("More", |ui| {
                        egui::menu::bar(ui, |ui| {
                            if ui.button("Save").clicked() {
                                debug!("Save file");
                                ui.close_menu();
                            }
                        });
                    });

                    ui.separator();

                    if ui.button("Exit").clicked() {
                        info!("Exiting application.");
                        ui.close_menu();

                        app_exit.send(AppExit::Success);
                    }
                });
            });
        })
        .response
        .rect
        .height();
}
