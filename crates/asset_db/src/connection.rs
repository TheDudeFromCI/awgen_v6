//! This module handles the SQLite database connection for asset management.

use std::marker::PhantomData;
use std::path::PathBuf;
// use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use bevy::asset::io::{AssetReaderError, AssetSourceEvent, AssetWriterError};
use bevy::prelude::*;
use crossbeam_channel::Sender;
use sqlite::{Connection, ConnectionThreadSafe, Value};

use crate::loaders::AwgenAsset;
use crate::module::{AssetModule, AssetModuleID};
use crate::record::{AssetRecord, AssetRecordID, ErasedAssetRecord};

/// Trait for obtaining the name of the asset database source.
pub trait AssetDatabaseName {
    /// The name of the database.
    fn database_name() -> &'static str;
}

/// Resource that encapsulates the SQLite connection for asset management.
///
/// This can be safely cloned to allow multiple systems to access the database
/// concurrently.
///
/// The `Src` generic parameter represents the database source, allowing for
/// multiple databases to be managed simultaneously.
#[derive(Resource)]
pub struct AssetDatabase<Src: AssetDatabaseName> {
    /// The thread-safe SQLite connection.
    connection: Arc<ConnectionThreadSafe>,

    /// Marker for the asset source type.
    _marker: PhantomData<Src>,

    /// List of active watchers monitoring the database for changes.
    watchers: Arc<RwLock<Vec<Sender<AssetSourceEvent>>>>,
}

impl<Src: AssetDatabaseName> Clone for AssetDatabase<Src> {
    fn clone(&self) -> Self {
        Self {
            connection: self.connection.clone(),
            _marker: PhantomData,
            watchers: self.watchers.clone(),
        }
    }
}

impl<Src: AssetDatabaseName> AssetDatabase<Src> {
    /// Creates a new [`AssetDatabase`] connection with the specified database
    /// file path. If the file does not exist, it will be created if possible.
    pub(crate) fn new<T: Into<PathBuf>>(path: T) -> Result<Self, AwgenDbError> {
        let connection = Connection::open_thread_safe(path.into())?;

        connection.execute(
            r#"
            CREATE TABLE IF NOT EXISTS modules (
                uuid TEXT PRIMARY KEY,
                name TEXT NOT NULL DEFAULT 'Unnamed'
            );
            CREATE TABLE IF NOT EXISTS assets (
                uuid TEXT PRIMARY KEY,
                type TEXT NOT NULL,
                path TEXT NOT NULL,
                module TEXT NOT NULL,
                data BLOB,
                preview BLOB,
                created INTEGER NOT NULL,
                last_modified INTEGER NOT NULL,
                FOREIGN KEY (module) REFERENCES modules (uuid)
            );
            "#,
        )?;

        Ok(Self {
            connection: Arc::new(connection),
            _marker: PhantomData,
            watchers: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Adds a new watcher to monitor the database for changes.
    pub(crate) fn add_watcher(&self, watcher: Sender<AssetSourceEvent>) {
        let mut watchers = self.watchers.write().unwrap();
        watchers.push(watcher);
    }

    /// Sends an event to all registered watchers.
    fn send_event(&self, event: AssetSourceEvent) {
        let watchers = self.watchers.read().unwrap();
        for sender in watchers.iter() {
            let _ = sender.send(event.clone());
        }
    }

    /// Retrieves all asset modules from the database.
    pub(crate) fn get_modules(&self) -> Result<Vec<AssetModule>, AwgenDbError> {
        let query = "SELECT uuid, name FROM modules";
        let mut modules = Vec::new();

        let mut statement = self.connection.prepare(query)?;
        while let Ok(sqlite::State::Row) = statement.next() {
            let uuid = statement.read::<String, _>("uuid")?;
            let name = statement.read::<String, _>("name")?;

            let Some(id) = AssetModuleID::from_string(&uuid) else {
                error!("Invalid UUID in asset database: {}", uuid);
                continue;
            };

            let module = AssetModule { id, name };
            modules.push(module);
        }

        Ok(modules)
    }

    /// Retrieves a specific asset module by its ID.
    pub(crate) fn get_module(
        &self,
        module_id: AssetModuleID,
    ) -> Result<Option<AssetModule>, AwgenDbError> {
        let query = "SELECT uuid, name FROM modules WHERE uuid = :uuid";

        let mut statement = self.connection.prepare(query)?;
        statement.bind((":uuid", module_id))?;

        if let Ok(sqlite::State::Row) = statement.next() {
            let uuid = statement.read::<String, _>("uuid")?;
            let name = statement.read::<String, _>("name")?;

            let Some(id) = AssetModuleID::from_string(&uuid) else {
                error!("Invalid AssetModuleID in asset database: {}", uuid);
                return Ok(None);
            };

            let module = AssetModule { id, name };
            Ok(Some(module))
        } else {
            Ok(None)
        }
    }

    /// Inserts (or updates) a new asset module into the database.
    pub(crate) fn insert_module(&self, module: &AssetModule) -> Result<(), AwgenDbError> {
        let query = "INSERT INTO modules (uuid, name) VALUES (:uuid, :name)";

        let mut statement = self.connection.prepare(query)?;
        statement.bind((":uuid", module.id))?;
        statement.bind((":name", module.name.as_str()))?;
        while let sqlite::State::Row = statement.next()? {}

        Ok(())
    }

    /// Removes an asset module from the database by its UUID.
    ///
    /// WARNING: This action will also delete *all* assets associated with this
    /// module.
    pub(crate) fn remove_module(&self, module: AssetModuleID) -> Result<(), AwgenDbError> {
        let assets = self.get_assets()?;
        for asset in assets {
            self.send_event(AssetSourceEvent::RemovedAsset(path_buf(
                asset.id,
                true,
                Image::type_name(),
            )));
            self.send_event(AssetSourceEvent::RemovedAsset(path_buf(
                asset.id,
                false,
                &asset.asset_type,
            )));
        }

        let module_query = "DELETE FROM modules WHERE uuid = :uuid";
        let mut statement = self.connection.prepare(module_query)?;
        statement.bind((":uuid", module))?;
        while let sqlite::State::Row = statement.next()? {}

        let asset_query = "DELETE FROM assets WHERE module = :module";
        let mut statement = self.connection.prepare(asset_query)?;
        statement.bind((":module", module))?;
        while let sqlite::State::Row = statement.next()? {}

        Ok(())
    }

    /// Retrieves a specific asset record by its ID, if it exists.
    ///
    /// This does not include the binary data or asset preview.
    pub(crate) fn get_asset(
        &self,
        id: AssetRecordID,
    ) -> Result<Option<ErasedAssetRecord>, AwgenDbError> {
        let query = r#"
            SELECT uuid, type, path, module, created, last_modified
            FROM assets
            WHERE uuid = :uuid;
        "#;

        let mut statement = self.connection.prepare(query)?;
        statement.bind((":uuid", id))?;
        statement.next()?;

        let uuid = statement.read::<String, _>("uuid")?;
        let asset_type = statement.read::<String, _>("type")?;
        let path = statement.read::<String, _>("path")?;
        let module_uuid = statement.read::<String, _>("module")?;
        let created = statement.read::<i64, _>("created")?;
        let last_modified = statement.read::<i64, _>("last_modified")?;

        let Some(id) = AssetRecordID::from_string(&uuid) else {
            error!("Invalid AssetRecordID in asset database: {}", uuid);
            return Ok(None);
        };

        let Some(module) = AssetModuleID::from_string(&module_uuid) else {
            error!("Invalid AssetModuleID in asset database: {}", module_uuid);
            return Ok(None);
        };

        let asset = ErasedAssetRecord {
            id,
            asset_type,
            pathname: PathBuf::from(path),
            module,
            created,
            last_modified,
        };
        Ok(Some(asset))
    }

    /// Retrieves all asset records of the given type from the database as
    /// partial records.
    ///
    /// Does not include preview or data fields.
    pub(crate) fn get_assets(&self) -> Result<Vec<ErasedAssetRecord>, AwgenDbError> {
        let query = "SELECT uuid, type, path, module, created, last_modified FROM assets";
        let mut assets = Vec::new();

        let mut statement = self.connection.prepare(query)?;
        while let Ok(sqlite::State::Row) = statement.next() {
            let uuid = statement.read::<String, _>("uuid")?;
            let asset_type = statement.read::<String, _>("type")?;
            let path = statement.read::<String, _>("path")?;
            let module_uuid = statement.read::<String, _>("module")?;
            let created = statement.read::<i64, _>("created")?;
            let last_modified = statement.read::<i64, _>("last_modified")?;

            let Some(id) = AssetRecordID::from_string(&uuid) else {
                error!("Invalid AssetRecordID in asset database: {}", uuid);
                continue;
            };

            let Some(module) = AssetModuleID::from_string(&module_uuid) else {
                error!("Invalid AssetModuleID in asset database: {}", module_uuid);
                continue;
            };

            let asset = ErasedAssetRecord {
                id,
                asset_type,
                pathname: PathBuf::from(path),
                module,
                created,
                last_modified,
            };

            assets.push(asset);
        }

        Ok(assets)
    }

    /// Inserts (or updates) a new asset record into the database.
    ///
    /// If the [`AssetRecord::created`] or [`AssetRecord::last_modified`] fields
    /// of the asset record are set to a negative value, it will be assigned
    /// to the current system time.
    pub(crate) fn insert_asset<A: AwgenAsset>(
        &self,
        asset: &AssetRecord<A>,
        data: &[u8],
    ) -> Result<(), AwgenDbError> {
        let module_query = r#"
            INSERT OR IGNORE INTO modules (uuid, name)
            VALUES (:module, 'Unnamed');
        "#;

        let asset_query = r#"
            INSERT INTO assets (uuid, type, path, module, created, last_modified, data)
            VALUES (:uuid, :type, :path, :module, :created, :last_modified, :data)
            ON CONFLICT(uuid) DO UPDATE SET
                type = excluded.type,
                path = excluded.path,
                module = excluded.module,
                created = excluded.created,
                last_modified = excluded.last_modified,
                data = excluded.data;
        "#;

        let mut created = asset.created;
        if created < 0 {
            created = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("System time set before UNIX EPOCH!")
                .as_millis() as i64;
        }

        let mut last_modified = asset.last_modified;
        if last_modified < 0 {
            last_modified = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("System time set before UNIX EPOCH!")
                .as_millis() as i64;
        }

        let pathname = asset.pathname.display().to_string();

        let mut statement = self.connection.prepare(module_query)?;
        statement.bind((":module", asset.module))?;
        while let sqlite::State::Row = statement.next()? {}

        let mut statement = self.connection.prepare(asset_query)?;
        statement.bind((":uuid", asset.id))?;
        statement.bind((":type", A::type_name()))?;
        statement.bind((":path", pathname.as_str()))?;
        statement.bind((":module", asset.module))?;
        statement.bind((":created", created))?;
        statement.bind((":last_modified", last_modified))?;
        statement.bind((":data", data))?;

        while let sqlite::State::Row = statement.next()? {}
        self.send_event(AssetSourceEvent::AddedAsset(path_buf(
            asset.id,
            false,
            A::type_name(),
        )));

        Ok(())
    }

    /// Sets the data blob for a specific asset by its ID.
    ///
    /// Calling this will overwrite any existing data for the asset and will
    /// update the `last_modified` timestamp.
    ///
    /// Note that this method does not validate the asset type; it is the
    /// caller's responsibility to ensure the data corresponds to the
    /// correct asset type.
    pub(crate) fn set_asset_data(
        &self,
        asset_id: AssetRecordID,
        data: &[u8],
    ) -> Result<(), AwgenDbError> {
        let record = self.get_asset(asset_id)?.ok_or_else(|| {
            AwgenDbError(sqlite::Error {
                code: Some(1),
                message: Some(format!("Asset with ID {} does not exist.", asset_id)),
            })
        })?;

        let query = r#"
            UPDATE assets
            SET data = :data,
                last_modified = :last_modified
            WHERE uuid = :uuid;
        "#;

        let last_modified = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("System time set before UNIX EPOCH!")
            .as_millis() as i64;

        let mut statement = self.connection.prepare(query)?;
        statement.bind((":uuid", asset_id))?;
        statement.bind((":last_modified", last_modified))?;
        statement.bind((":data", data))?;

        while let sqlite::State::Row = statement.next()? {}
        self.send_event(AssetSourceEvent::ModifiedAsset(path_buf(
            asset_id,
            false,
            &record.asset_type,
        )));

        Ok(())
    }

    /// Sets the data preview for a specific asset by its ID.
    ///
    /// Calling this will overwrite any existing preview for the asset and will
    /// update the `last_modified` timestamp.
    ///
    /// Passing a `None` value will remove the existing preview.
    pub(crate) fn set_asset_preview(
        &self,
        asset_id: AssetRecordID,
        preview: Option<&[u8]>,
    ) -> Result<(), AwgenDbError> {
        let query = r#"
            UPDATE assets
            SET preview = :preview,
                last_modified = :last_modified
            WHERE uuid = :uuid;
        "#;

        let last_modified = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("System time set before UNIX EPOCH!")
            .as_millis() as i64;

        let mut statement = self.connection.prepare(query)?;
        statement.bind((":uuid", asset_id))?;
        statement.bind((":last_modified", last_modified))?;

        if let Some(preview) = preview {
            statement.bind((":preview", preview))?;
        } else {
            statement.bind((":preview", Value::Null))?;
        }

        while let sqlite::State::Row = statement.next()? {}

        self.send_event(AssetSourceEvent::ModifiedAsset(path_buf(
            asset_id,
            true,
            Image::type_name(),
        )));

        Ok(())
    }

    /// Retrieves the data blob for a specific asset by its ID.
    pub(crate) fn get_asset_data(
        &self,
        asset_id: AssetRecordID,
    ) -> Result<Option<Vec<u8>>, AwgenDbError> {
        let query = "SELECT data FROM assets WHERE uuid = :uuid";

        let mut statement = self.connection.prepare(query)?;
        statement.bind((":uuid", asset_id))?;

        if let Ok(sqlite::State::Row) = statement.next() {
            let data = statement.read::<Vec<u8>, _>("data")?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    /// Retrieves the preview data for a specific asset by its ID.
    pub(crate) fn get_asset_preview(
        &self,
        asset_id: AssetRecordID,
    ) -> Result<Option<Vec<u8>>, AwgenDbError> {
        let query = "SELECT preview FROM assets WHERE uuid = :uuid";

        let mut statement = self.connection.prepare(query)?;
        statement.bind((":uuid", asset_id))?;

        if let Ok(sqlite::State::Row) = statement.next() {
            let preview = statement.read::<Vec<u8>, _>("preview")?;
            if preview.is_empty() {
                Ok(None)
            } else {
                Ok(Some(preview))
            }
        } else {
            Ok(None)
        }
    }

    /// Removes an asset record from the database by its ID.
    pub(crate) fn remove_asset(&self, asset_id: AssetRecordID) -> Result<(), AwgenDbError> {
        let Some(record) = self.get_asset(asset_id)? else {
            return Ok(());
        };

        let query = "DELETE FROM assets WHERE uuid = :uuid";

        let mut statement = self.connection.prepare(query)?;
        statement.bind((":uuid", asset_id))?;
        while let sqlite::State::Row = statement.next()? {}

        self.send_event(AssetSourceEvent::RemovedAsset(path_buf(
            asset_id,
            true,
            Image::type_name(),
        )));
        self.send_event(AssetSourceEvent::RemovedAsset(path_buf(
            asset_id,
            false,
            &record.asset_type,
        )));

        Ok(())
    }
}

/// An error that can occur while interacting with the database.
#[derive(Debug, thiserror::Error)]
#[error("Failed to connect with database: {0}")]
pub struct AwgenDbError(#[from] pub sqlite::Error);

impl From<AwgenDbError> for AssetReaderError {
    fn from(value: AwgenDbError) -> Self {
        AssetReaderError::Io(Arc::new(std::io::Error::new(
            std::io::ErrorKind::ConnectionAborted,
            format!(
                "Error {}: {}",
                value.0.code.unwrap_or(-1),
                value.0.message.unwrap_or("Unknown error".into())
            ),
        )))
    }
}

impl From<AwgenDbError> for AssetWriterError {
    fn from(value: AwgenDbError) -> Self {
        AssetWriterError::Io(std::io::Error::new(
            std::io::ErrorKind::ConnectionAborted,
            format!(
                "Error {}: {}",
                value.0.code.unwrap_or(-1),
                value.0.message.unwrap_or("Unknown error".into())
            ),
        ))
    }
}

/// Generates a path buffer for the asset data or preview based on the asset ID
/// and whether it's a preview or not.
fn path_buf(id: AssetRecordID, is_preview: bool, asset_type: &str) -> PathBuf {
    let format = match is_preview {
        true => "preview",
        false => "data",
    };
    PathBuf::from(format!("{}.{}.{}", id, format, asset_type))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDatabase;
    impl AssetDatabaseName for TestDatabase {
        fn database_name() -> &'static str {
            "test_database"
        }
    }

    fn asset() -> AssetRecord<Image> {
        AssetRecord {
            id: AssetRecordID::new(),
            pathname: PathBuf::from("test/asset.png"),
            module: AssetModuleID::new(),
            created: 100,
            last_modified: 100,
            _marker: PhantomData,
        }
    }

    fn module() -> AssetModule {
        AssetModule {
            id: AssetModuleID::new(),
            name: "Test Module".into(),
        }
    }

    #[test]
    fn test_database_connection() {
        let db = AssetDatabase::<TestDatabase>::new(":memory:");
        assert!(db.is_ok());
    }

    #[test]
    fn test_insert_and_get_module() {
        let db = AssetDatabase::<TestDatabase>::new(":memory:").unwrap();

        let module = module();
        db.insert_module(&module).unwrap();

        let fetched_module = db.get_module(module.id).unwrap().unwrap();
        assert_eq!(fetched_module.id, module.id);
        assert_eq!(fetched_module.name, module.name);
    }

    #[test]
    fn test_insert_and_get_asset() {
        let db = AssetDatabase::<TestDatabase>::new(":memory:").unwrap();

        let module = module();
        db.insert_module(&module).unwrap();

        let asset_id = AssetRecordID::new();
        let asset = AssetRecord {
            id: asset_id,
            module: module.id,
            ..asset()
        };
        db.insert_asset(&asset, &[1, 2, 3]).unwrap();

        let record = db.get_asset(asset_id).unwrap().unwrap();

        let erased_asset = ErasedAssetRecord {
            id: asset.id,
            asset_type: Image::type_name().to_string(),
            pathname: asset.pathname.clone(),
            module: asset.module,
            created: asset.created,
            last_modified: asset.last_modified,
        };

        assert_eq!(record, erased_asset);

        let fetched_module = db.get_module(record.module).unwrap().unwrap();
        assert_eq!(fetched_module.id, module.id);
        assert_eq!(fetched_module.name, module.name);
    }

    #[test]
    fn timestamp_auto_update() {
        let db = AssetDatabase::<TestDatabase>::new(":memory:").unwrap();

        let module = module();
        db.insert_module(&module).unwrap();

        let asset_id = AssetRecordID::new();
        let asset = AssetRecord {
            id: asset_id,
            module: module.id,
            created: -1,
            last_modified: -1,
            ..asset()
        };
        db.insert_asset(&asset, &[1, 2, 3]).unwrap();

        let record = db.get_asset(asset_id).unwrap().unwrap();
        assert!(record.created > 0);
        assert!(record.last_modified > 0);
    }

    #[test]
    fn multiple_assets() {
        let db = AssetDatabase::<TestDatabase>::new(":memory:").unwrap();

        let module = module();
        db.insert_module(&module).unwrap();

        for _ in 0 .. 5 {
            let asset = AssetRecord {
                module: module.id,
                ..asset()
            };
            db.insert_asset(&asset, &[1, 2, 3]).unwrap();
        }

        let assets = db.get_assets().unwrap();
        assert_eq!(assets.len(), 5);
    }

    #[test]
    fn update_data() {
        let db = AssetDatabase::<TestDatabase>::new(":memory:").unwrap();

        let module = module();
        db.insert_module(&module).unwrap();

        let asset_id = AssetRecordID::new();
        let asset = AssetRecord {
            id: asset_id,
            module: module.id,
            ..asset()
        };

        let data = vec![1, 2, 3, 4, 5];
        db.insert_asset(&asset, &data).unwrap();

        let preview = vec![10, 20, 30];
        db.set_asset_preview(asset_id, Some(&preview)).unwrap();

        let fetched_data = db.get_asset_data(asset_id).unwrap().unwrap();
        assert_eq!(fetched_data, data);

        let fetched_preview = db.get_asset_preview(asset_id).unwrap().unwrap();
        assert_eq!(fetched_preview, preview);
    }

    #[test]
    fn asset_with_non_existent_module() {
        let db = AssetDatabase::<TestDatabase>::new(":memory:").unwrap();
        let module_id = AssetModuleID::new();

        let asset_id = AssetRecordID::new();
        let asset = AssetRecord {
            id: asset_id,
            module: module_id,
            ..asset()
        };
        db.insert_asset(&asset, &[1, 2, 3]).unwrap();

        let fetched_module = db.get_module(module_id).unwrap().unwrap();
        assert_eq!(fetched_module.id, module_id);
        assert_eq!(fetched_module.name, "Unnamed");
    }

    #[test]
    fn delete_module_clears_assets() {
        let db = AssetDatabase::<TestDatabase>::new(":memory:").unwrap();

        let module1 = module();
        db.insert_module(&module1).unwrap();

        let module2 = module();
        db.insert_module(&module2).unwrap();

        for _ in 0 .. 3 {
            let asset = AssetRecord {
                module: module1.id,
                ..asset()
            };
            db.insert_asset(&asset, &[1, 2, 3]).unwrap();
        }

        for _ in 0 .. 3 {
            let asset = AssetRecord {
                module: module2.id,
                ..asset()
            };
            db.insert_asset(&asset, &[1, 2, 3]).unwrap();
        }

        let assets = db.get_assets().unwrap();
        assert_eq!(assets.len(), 6);

        db.remove_module(module1.id).unwrap();

        let assets = db.get_assets().unwrap();
        assert_eq!(assets.len(), 3);
    }
}
