//! This module adds a file picker dialog utility to the editor.

use bevy::tasks::futures_lite::future;
use bevy::tasks::{IoTaskPool, Task};

/// A file picker dialog that allows the user to select a file or folder.
/// Automatically opens a dialog on creation and handles async Bevy tasks.
pub struct FilePickerDialog {
    /// The async task that handles the file picker dialog.
    task: Task<FileSelectionResult>,

    /// The result of the file picker dialog.
    value: Option<FileSelectionResult>,
}

impl FilePickerDialog {
    /// Creates a new file picker dialog that allows the user to select a
    /// folder.
    pub fn select_folder(text: String) -> Self {
        let task = IoTaskPool::get().spawn(async move {
            match tinyfiledialogs::select_folder_dialog(&text, "") {
                Some(path) => FileSelectionResult::Path(path),
                None => FileSelectionResult::Canceled,
            }
        });

        Self { task, value: None }
    }

    /// Creates a new file picker dialog that allows the user to select a file.
    pub fn select_file(text: String, filter: Vec<String>, desc: String) -> Self {
        let task = IoTaskPool::get().spawn(async move {
            let filter: Vec<&str> = filter.iter().map(|s| s.as_str()).collect();
            match tinyfiledialogs::open_file_dialog(&text, "", Some((&filter, &desc))) {
                Some(path) => FileSelectionResult::Path(path),
                None => FileSelectionResult::Canceled,
            }
        });

        Self { task, value: None }
    }

    /// Polls the file picker dialog to check if the user has made a selection.
    pub fn poll(&mut self) -> &Option<FileSelectionResult> {
        if self.value.is_none() {
            self.value = future::block_on(future::poll_once(&mut self.task));
        }

        &self.value
    }

    /// Returns `true` if the file picker dialog is still open.
    pub fn is_open(&self) -> bool {
        self.value.is_none()
    }
}

/// The result of a file selection dialog.
pub enum FileSelectionResult {
    /// The user selected a file or folder.
    Path(String),

    /// The user canceled the dialog.
    Canceled,
}
