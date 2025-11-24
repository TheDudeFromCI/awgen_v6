//! this module implements the systems for the Awgen asset database plugin.

use bevy::prelude::*;
use bevy::tasks::Task;
use bevy::tasks::futures_lite::future;

use crate::connection::AssetDatabaseName;
use crate::loaders::{AssetDataError, ImagePreviewData};
use crate::param::AwgenAssets;
use crate::record::AssetRecordID;

/// System to update asset previews for assets whose preview generation tasks
/// have completed.
pub(super) fn update_previews<Src>(
    mut results: Local<Vec<(AssetRecordID, Result<ImagePreviewData, AssetDataError>)>>,
    mut assets: AwgenAssets<Src>,
) where
    Src: AssetDatabaseName + Send + Sync + 'static,
{
    assets
        .preview_tasks_mut()
        .retain_mut(|(id, task)| match poll(task) {
            Some(result) => {
                results.push((*id, result));
                false
            }
            None => true,
        });

    for (id, result) in results.drain(..) {
        match result {
            Ok(preview) => {
                if let Err(e) = assets.save_asset_preview(id, Some(preview)) {
                    error!("Failed to save preview for asset {}: {}", id, e);
                }
            }
            Err(e) => {
                error!("Failed to generate preview for asset {}: {}", id, e);
                if let Err(e) = assets.save_asset_preview(id, None) {
                    error!("Failed to remove old preview for asset {}: {}", id, e);
                }
            }
        }
    }
}

/// A small helper function to poll a Bevy task.
///
/// If the task is complete, it returns `Some` with the result; otherwise,
/// it returns `None`.
fn poll<T>(task: &mut Task<T>) -> Option<T> {
    future::block_on(future::poll_once(task))
}
