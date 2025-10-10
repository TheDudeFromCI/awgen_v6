//! File drop handling for Awgen.

use bevy::prelude::*;

use crate::scripts::{PacketOut, ScriptEngine};

/// Handles file drop events for Awgen, forwarding the event to the script
/// engine.
pub(super) fn handle_file_drop(
    mut file_drop_evs: MessageReader<FileDragAndDrop>,
    sockets: Res<ScriptEngine>,
) {
    for ev in file_drop_evs.read() {
        match ev {
            FileDragAndDrop::DroppedFile { path_buf, .. } => {
                if let Err(err) = sockets.send(PacketOut::FileDrop {
                    path: path_buf.to_string_lossy().to_string(),
                }) {
                    error!("Failed to send file drop event to script engine: {}", err);
                }
            }
            FileDragAndDrop::HoveredFile { .. } => {}
            FileDragAndDrop::HoveredFileCanceled { .. } => {}
        }
    }
}
