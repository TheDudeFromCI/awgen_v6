//! This module implements the [`AssetRecord`] structure and related types.

use std::fmt;
use std::path::PathBuf;

use sqlite::{BindableWithIndex, ParameterIndex, Statement};
use uuid::Uuid;

use crate::loaders::AwgenAsset;
use crate::module::AssetModuleID;

/// Unique identifier for an asset record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetRecordID(Uuid);

impl AssetRecordID {
    /// Creates a new `AssetRecordID` with a generated UUID.
    #[allow(clippy::new_without_default)]
    pub(crate) fn new() -> Self {
        AssetRecordID(Uuid::new_v4())
    }

    /// Creates an `AssetRecordID` from a string representation of a UUID.
    pub(crate) fn from_string<S: AsRef<str>>(s: S) -> Option<Self> {
        Uuid::parse_str(s.as_ref()).ok().map(AssetRecordID)
    }
}

impl fmt::Display for AssetRecordID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BindableWithIndex for AssetRecordID {
    fn bind<T: ParameterIndex>(
        self,
        statement: &mut Statement,
        index: T,
    ) -> Result<(), sqlite::Error> {
        let uuid = self.0.to_string();
        uuid.as_str().bind(statement, index)
    }
}

/// Represents an asset record in the asset database with the type erased.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErasedAssetRecord {
    /// Unique identifier for the asset.
    pub id: AssetRecordID,

    /// Type of the asset.
    pub asset_type: String,

    /// The pathname of the asset within the database. This does not have to be
    /// unique.
    pub pathname: PathBuf,

    /// Associated module ID.
    pub module: AssetModuleID,

    /// Timestamp of creation (Unix epoch).
    ///
    /// If set to a negative value, it indicates that the creation time
    /// should be assigned to the current system time when inserting the
    /// record in the database.
    pub created: i64,

    /// Timestamp of the last modification (Unix epoch).
    ///
    /// If set to a negative value, it indicates that the last modified time
    /// should be assigned to the current system time when inserting or updating
    /// the record in the database.
    pub last_modified: i64,
}

/// Represents an asset record in the asset database.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetRecord<A: AwgenAsset> {
    /// Unique identifier for the asset.
    pub id: AssetRecordID,

    /// The pathname of the asset within the database. This does not have to be
    /// unique.
    pub pathname: PathBuf,

    /// Associated module ID.
    pub module: AssetModuleID,

    /// Timestamp of creation (Unix epoch).
    ///
    /// If set to a negative value, it indicates that the creation time
    /// should be assigned to the current system time when inserting the
    /// record in the database.
    pub created: i64,

    /// Timestamp of the last modification (Unix epoch).
    ///
    /// If set to a negative value, it indicates that the last modified time
    /// should be assigned to the current system time when inserting or updating
    /// the record in the database.
    pub last_modified: i64,

    /// Marker for the asset type.
    pub _marker: std::marker::PhantomData<A>,
}
