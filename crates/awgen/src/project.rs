//! This module implements the ProjectPlugin, which handles the project
//! management for the editor.

use std::fs;
use std::path::PathBuf;

use bevy::prelude::*;

/// This plugin provides the project management for the editor.
pub struct ProjectPlugin;
impl Plugin for ProjectPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_event::<ProjectActionEvent>()
            .add_systems(Update, handle_project_action);
    }
}

/// This resource stores metadata about the currently loaded project.
#[derive(Default, Resource)]
pub struct ProjectSettings {
    /// The path to the project folder.
    pub data_folder: PathBuf,

    /// The path to the project settings file.
    pub project_settings: PathBuf,
}

/// An action to global action to perform on a project.
#[derive(Debug, Event, Clone, PartialEq, Eq, Hash)]
pub enum ProjectActionEvent {
    /// Create a new project at the indicated path.
    New(String),

    /// Open the project at the indicated path.
    Open(String),

    /// Close the currently open project.
    Close,
}

fn handle_project_action(
    project_settings: Option<Res<ProjectSettings>>,
    mut project_action_evs: EventReader<ProjectActionEvent>,
    mut commands: Commands,
) {
    for ev in project_action_evs.read() {
        match ev {
            ProjectActionEvent::New(path) => {
                if project_settings.is_some() {
                    error!("Cannot create a new project while another project is open.");
                    continue;
                }

                let project_folder_path = PathBuf::from(path);
                match fs::exists(&project_folder_path) {
                    Ok(true) => {
                        if project_folder_path.is_dir() {
                            info!("Creating a new project at path: {}", &path);

                            let project_settings_path = project_folder_path.join("project.awgen");

                            // TODO: Create project settings file via SQLite
                            if let Err(err) = fs::write(&project_settings_path, "") {
                                error!("Failed to create project settings file: {}", err);
                                continue;
                            }

                            commands.insert_resource(ProjectSettings {
                                data_folder: project_folder_path,
                                project_settings: project_settings_path,
                            });
                        } else {
                            error!("Project path at {} is not a directory.", &path);
                        }
                    }
                    Ok(false) => {
                        error!(
                            "Cannot create a new project at a non-existent path: {}",
                            &path
                        );
                    }
                    Err(err) => {
                        error!("Failed to check if project path exists: {}", err);
                    }
                }
            }
            ProjectActionEvent::Open(path) => {
                if project_settings.is_some() {
                    error!("Cannot open a project while another project is open.");
                    continue;
                }

                let mut project_settings_path = PathBuf::from(path);
                match fs::exists(&project_settings_path) {
                    Ok(true) => {
                        if project_settings_path.is_file() {
                            project_settings_path = match project_settings_path.canonicalize() {
                                Ok(path) => path,
                                Err(err) => {
                                    error!("Failed to canonicalize project path: {}", err);
                                    continue;
                                }
                            };

                            let project_folder_path = match project_settings_path.parent() {
                                Some(path) => path.to_path_buf(),
                                None => {
                                    error!("Failed to get parent path of project path.");
                                    continue;
                                }
                            };

                            if project_settings_path.extension() == Some("awgen".as_ref()) {
                                info!("Opening project at path: {}", &path);
                                commands.insert_resource(ProjectSettings {
                                    data_folder: project_folder_path,
                                    project_settings: project_settings_path,
                                });
                            } else {
                                error!("Project file at {} is not an '.awgen' file.", &path);
                            }
                        } else {
                            error!("Project path at {} is not a file.", &path);
                        }
                    }
                    Ok(false) => {
                        error!("Cannot open a project at a non-existent path: {}", &path);
                    }
                    Err(err) => {
                        error!("Failed to check if project path exists: {}", err);
                    }
                }
            }
            ProjectActionEvent::Close => {
                if project_settings.is_none() {
                    error!("Cannot close a project when no project is open.");
                    continue;
                }

                commands.remove_resource::<ProjectSettings>();
            }
        }
    }
}
