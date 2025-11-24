//! This module implements the [`AssetDatabase`] system parameter for
//! accessing Awgen asset databases within Bevy systems.

use std::path::PathBuf;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::tasks::Task;

use crate::loaders::{AssetDataError, AwgenAsset, ImagePreviewData};
use crate::module::{AssetModule, AssetModuleID};
use crate::prelude::{AssetDatabase, AssetDatabaseName, AwgenDbError};
use crate::record::{AssetRecord, AssetRecordID, ErasedAssetRecord};

/// A resource to track assets that need their previews updated.
#[derive(Debug, Default, Resource)]
pub struct AssetDatabaseTasks {
    /// Tasks for generating asset previews.
    preview_generation: Vec<(
        AssetRecordID,
        Task<Result<ImagePreviewData, AssetDataError>>,
    )>,
}

/// System parameter for accessing the Awgen asset database.
#[derive(SystemParam)]
pub struct AwgenAssets<'w, Src>
where
    Src: AssetDatabaseName + Send + Sync + 'static,
{
    /// The Bevy asset server used to load assets.
    asset_server: Res<'w, AssetServer>,

    /// The Awgen asset database connection.
    db: Res<'w, AssetDatabase<Src>>,

    /// Tasks for managing asset database operations.
    tasks: ResMut<'w, AssetDatabaseTasks>,
}

impl<'w, Src> AwgenAssets<'w, Src>
where
    Src: AssetDatabaseName + Send + Sync + 'static,
{
    /// Loads an asset of type `T` from the specified source and asset record
    /// ID.
    ///
    /// If the asset preview is updated in the database, the handle will
    /// automatically reflect the new changes when reloaded by Bevy's asset
    /// watcher system.
    pub fn load_asset<A: AwgenAsset>(&self, id: AssetRecordID) -> Handle<A> {
        debug!("Loading asset {} of type {}", id, A::type_name());
        let path = format!("{}://{}.data.{}", Src::database_name(), id, A::type_name());
        self.asset_server.load(path)
    }

    /// Loads the preview image for an asset from the specified source and
    /// asset record ID.
    ///
    /// If the asset does not have a preview image, the loaded image will be
    /// a default 4x4 transparent image.
    ///
    /// If the asset preview is updated in the database, the handle will
    /// automatically reflect the new changes when reloaded by Bevy's asset
    /// watcher system.
    pub fn load_asset_preview(&self, id: AssetRecordID) -> Handle<Image> {
        debug!("Loading preview for asset {}", id);
        let path = format!(
            "{}://{}.preview.{}",
            Src::database_name(),
            id,
            Image::type_name()
        );
        self.asset_server.load(path)
    }

    /// Lists all asset records available in the asset database.
    ///
    /// This method is very slow and should be used sparingly. Values should be
    /// cached where possible.
    pub fn list_assets(&self) -> Result<Vec<ErasedAssetRecord>, AwgenAssetsError> {
        // TODO: Move this impl into the task pool?
        debug!("Fetch all asset records from the database");
        Ok(self.db.get_assets()?)
    }

    /// Lists all asset modules available in the asset database.
    ///
    /// This method is very slow and should be used sparingly. Values should be
    /// cached where possible.
    pub fn list_modules(&self) -> Result<Vec<AssetModule>, AwgenAssetsError> {
        // TODO: Move this impl into the task pool?
        debug!("Fetch all asset modules from the database");
        Ok(self.db.get_modules()?)
    }

    /// Retrieves the asset module with the specified ID.
    ///
    /// This method is very slow and should be used sparingly. Values should be
    /// cached where possible.
    pub fn get_module(&self, id: AssetModuleID) -> Result<Option<AssetModule>, AwgenAssetsError> {
        debug!("Fetch asset module {} from the database", id);
        Ok(self.db.get_module(id)?)
    }

    /// Creates a new asset module with the given name.
    ///
    /// This method requires a Database query and is very slow.
    pub fn create_module(&self, name: &str) -> Result<AssetModuleID, AwgenAssetsError> {
        // TODO: Move this impl into the task pool?

        let id = AssetModuleID::new();
        let module = AssetModule {
            id,
            name: name.to_string(),
        };

        self.db.insert_module(&module)?;
        info!("Created new asset module {}: {}", id, name);

        Ok(id)
    }

    /// Removes the asset module with the specified ID.
    ///
    /// This method requires a Database query and is very slow.
    pub fn remove_module(&self, id: AssetModuleID) -> Result<(), AwgenAssetsError> {
        // TODO: Move this impl into the task pool?

        self.db.remove_module(id)?;
        info!("Removed asset module {}", id);

        Ok(())
    }

    /// Creates a new asset of type `A` in the specified asset module.
    ///
    /// The `name` parameter is currently unused and can be set to `None`.
    ///
    /// This method requires a Database query and is very slow.
    pub fn create_asset<A: AwgenAsset, P: Into<PathBuf>>(
        &mut self,
        pathname: P,
        module: AssetModuleID,
        asset: &A,
    ) -> Result<AssetRecordID, AwgenAssetsError> {
        // TODO: Move this impl into the task pool?

        let id = AssetRecordID::new();
        let record = AssetRecord::<A> {
            id,
            pathname: pathname.into(),
            module,
            created: -1,
            last_modified: -1,
            _marker: std::marker::PhantomData,
        };

        let data = asset.save()?;
        self.db.insert_asset(&record, &data)?;

        info!(
            "Created new asset {} \"{}\" of type {} in module {}",
            id,
            record.pathname.display(),
            A::type_name(),
            module
        );

        self.update_preview(id, asset);
        Ok(id)
    }

    /// Saves the given asset of type `A` into the asset database with the
    /// specified asset record ID, updating the existing asset data.
    ///
    /// This method will trigger the asset preview to be regenerated.
    ///
    /// This method requires a Database query and is very slow.
    pub fn update_asset<A: AwgenAsset>(
        &mut self,
        id: AssetRecordID,
        asset: &A,
    ) -> Result<(), AwgenAssetsError> {
        // TODO: Move this impl into the task pool?

        let Some(record) = self.db.get_asset(id)? else {
            return Err(AwgenAssetsError::MissingAsset(id));
        };

        if record.asset_type != A::type_name() {
            return Err(AwgenAssetsError::WrongType(
                A::type_name().to_string(),
                record.asset_type,
            ));
        }

        let data = asset.save()?;
        self.db.set_asset_data(id, &data)?;

        info!("Updated asset {} of type {}", id, A::type_name());

        self.update_preview(id, asset);

        Ok(())
    }

    /// Saves the preview image for an asset into the asset database with the
    /// specified asset record ID.
    ///
    /// This method requires a Database query and is very slow.
    ///
    /// If `preview` is `None`, the preview will be removed.
    pub(crate) fn save_asset_preview(
        &self,
        id: AssetRecordID,
        preview: Option<ImagePreviewData>,
    ) -> Result<(), AwgenAssetsError> {
        // TODO: Move this impl into the task pool?

        if let Some(preview) = preview {
            let image: Image = preview.into();
            let data = image.save()?;
            self.db.set_asset_preview(id, Some(&data))?;
            info!("Updated preview for asset {}", id);
        } else {
            self.db.set_asset_preview(id, None)?;
            info!("Reset preview for asset {}", id);
        }

        Ok(())
    }

    /// This method spawns a background task to generate a new preview image for
    /// the asset with the specified asset record ID, using the provided asset
    /// data.
    fn update_preview<A: AwgenAsset>(&mut self, id: AssetRecordID, asset: &A) {
        debug!("Spawning preview generation task for asset {}", id);
        let task = A::generate_preview(asset);
        self.tasks.preview_generation.push((id, task));
    }

    /// Deletes the asset with the specified asset record ID from the asset
    /// database.
    ///
    /// This method requires a Database query and is very slow.
    pub fn delete_asset(&self, id: AssetRecordID) -> Result<(), AwgenAssetsError> {
        // TODO: Move this impl into the task pool?

        info!("Deleting asset {}", id);
        self.db.remove_asset(id)?;
        Ok(())
    }

    /// Provides mutable access to the preview generation tasks.
    pub(crate) fn preview_tasks_mut(
        &mut self,
    ) -> &mut Vec<(
        AssetRecordID,
        Task<Result<ImagePreviewData, AssetDataError>>,
    )> {
        &mut self.tasks.preview_generation
    }
}

/// Error type for Awgen asset database operations.
#[derive(Debug, thiserror::Error)]
pub enum AwgenAssetsError {
    /// Error saving or loading asset data.
    #[error("Asset data error: {0}")]
    Data(#[from] AssetDataError),

    /// Error interacting with the Awgen asset database.
    #[error("Asset database error: {0}")]
    Database(#[from] AwgenDbError),

    /// The asset type does not match the expected type.
    #[error("Asset type mismatch: expected '{0}', found '{1}'")]
    WrongType(String, String),

    /// The specified asset record was not found.
    #[error("Asset record not found: {0}")]
    MissingAsset(AssetRecordID),
}
