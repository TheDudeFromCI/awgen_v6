//! This module implements the [`AssetDatabase`] system parameter for
//! accessing Awgen asset databases within Bevy systems.

use std::path::PathBuf;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::loaders::{AssetDataError, AwgenAsset};
use crate::module::{AssetModule, AssetModuleID};
use crate::prelude::{AssetDatabase, AssetDatabaseName, AwgenDbError};
use crate::record::{AssetRecord, AssetRecordID, ErasedAssetRecord};

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
}

impl<'w, Src> AwgenAssets<'w, Src>
where
    Src: AssetDatabaseName + Send + Sync + 'static,
{
    /// Loads an asset of type `T` from the specified source and asset record
    /// ID.
    pub fn load_asset<A: AwgenAsset>(&self, id: AssetRecordID) -> Handle<A> {
        let path = format!("{}://{}.data.{}", Src::database_name(), id, A::type_name());
        self.asset_server.load(path)
    }

    /// Loads the preview image for an asset from the specified source and
    /// asset record ID.
    pub fn load_asset_preview(&self, id: AssetRecordID) -> Handle<Image> {
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
        Ok(self.db.get_assets()?)
    }

    /// Lists all asset modules available in the asset database.
    ///
    /// This method is very slow and should be used sparingly. Values should be
    /// cached where possible.
    pub fn list_modules(&self) -> Result<Vec<AssetModule>, AwgenAssetsError> {
        // TODO: Move this impl into the task pool?
        Ok(self.db.get_modules()?)
    }

    /// Retrieves the asset module with the specified ID.
    ///
    /// This method is very slow and should be used sparingly. Values should be
    /// cached where possible.
    pub fn get_module(&self, id: AssetModuleID) -> Result<Option<AssetModule>, AwgenAssetsError> {
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
        Ok(id)
    }

    /// Removes the asset module with the specified ID.
    ///
    /// This method requires a Database query and is very slow.
    pub fn remove_module(&self, id: AssetModuleID) -> Result<(), AwgenAssetsError> {
        // TODO: Move this impl into the task pool?

        self.db.remove_module(id)?;
        Ok(())
    }

    /// Creates a new asset of type `A` in the specified asset module.
    ///
    /// The `name` parameter is currently unused and can be set to `None`.
    ///
    /// This method requires a Database query and is very slow.
    pub fn create_asset<A: AwgenAsset, P: Into<PathBuf>>(
        &self,
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

        Ok(id)
    }

    /// Saves the given asset of type `A` into the asset database with the
    /// specified asset record ID, updating the existing asset data.
    ///
    /// This method requires a Database query and is very slow.
    pub fn update_asset<A: AwgenAsset>(
        &self,
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

        Ok(())
    }

    /// Saves the preview image for an asset into the asset database with the
    /// specified asset record ID.
    ///
    /// This method requires a Database query and is very slow.
    ///
    /// If `preview` is `None`, the preview will be removed.
    pub fn save_asset_preview(
        &self,
        id: AssetRecordID,
        preview: Option<&Image>,
    ) -> Result<(), AwgenAssetsError> {
        // TODO: Move this impl into the task pool?

        if let Some(preview) = preview {
            let data = preview.save()?;
            self.db.set_asset_preview(id, Some(&data))?;
        } else {
            self.db.set_asset_preview(id, None)?;
        }

        Ok(())
    }

    /// Deletes the asset with the specified asset record ID from the asset
    /// database.
    ///
    /// This method requires a Database query and is very slow.
    pub fn delete_asset(&self, id: AssetRecordID) -> Result<(), AwgenAssetsError> {
        // TODO: Move this impl into the task pool?

        self.db.remove_asset(id)?;
        Ok(())
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
