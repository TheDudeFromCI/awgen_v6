//! The asset source implementation for Awgen asset database.

use std::path::Path;

use bevy::asset::io::{AssetReader, AssetReaderError, AssetWatcher, PathStream, Reader, VecReader};
use bevy::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

use crate::connection::AssetDatabase;
use crate::loaders::AwgenAsset;
use crate::prelude::AssetDatabaseName;
use crate::record::AssetRecordID;

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"^([a-f0-9\-]{36})\.(data|preview).([a-zA-Z0-9_-]+)$").unwrap();
}

/// Asset source that reads and writes assets to the [`AssetDatabase`].
pub struct AwgenDbSource<Src>
where
    Src: AssetDatabaseName + Send + Sync + 'static,
{
    /// The asset database connection.
    pub database: AssetDatabase<Src>,
}

impl<Src> Clone for AwgenDbSource<Src>
where
    Src: AssetDatabaseName + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            database: self.database.clone(),
        }
    }
}

impl<Src> AssetReader for AwgenDbSource<Src>
where
    Src: AssetDatabaseName + Send + Sync + 'static,
{
    async fn read<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        let path_str = path.to_string_lossy().to_string();
        let Some(captures) = REGEX.captures(&path_str) else {
            return Err(AssetReaderError::NotFound(path.to_path_buf()));
        };

        let uuid = &captures[1];
        let asset_type = &captures[3];
        let is_preview = match &captures[2] {
            "preview" => true,
            "data" => false,
            _ => unreachable!(),
        };

        let asset_id = AssetRecordID::from_string(uuid)
            .ok_or(AssetReaderError::NotFound(path.to_path_buf()))?;

        let data = match is_preview {
            true => {
                if asset_type != Image::type_name() {
                    return Err(AssetReaderError::NotFound(path.to_path_buf()));
                }
                self.database.get_asset_preview(asset_id)
            }
            false => {
                let Some(record) = self.database.get_asset(asset_id)? else {
                    return Err(AssetReaderError::NotFound(path.to_path_buf()));
                };

                if record.asset_type != asset_type {
                    return Err(AssetReaderError::NotFound(path.to_path_buf()));
                }

                self.database.get_asset_data(asset_id)
            }
        };

        match data {
            Ok(Some(data)) => Ok(VecReader::new(data)),
            Ok(None) => Err(AssetReaderError::NotFound(path.to_path_buf())),
            Err(e) => Err(e.into()),
        }
    }

    async fn read_meta<'a>(&'a self, path: &'a Path) -> Result<VecReader, AssetReaderError> {
        Err(AssetReaderError::NotFound(path.to_path_buf()))
    }

    async fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Result<Box<PathStream>, AssetReaderError> {
        Err(AssetReaderError::NotFound(path.to_path_buf()))
    }

    async fn is_directory<'a>(&'a self, _: &'a Path) -> Result<bool, AssetReaderError> {
        Ok(false)
    }
}

/// Watcher that monitors the asset database for changes.
pub struct AwgenDbWatcher;
impl AssetWatcher for AwgenDbWatcher {}
