//! This module implements the [`AssetModule`] struct and related functionality.

use std::fmt;

use sqlite::{BindableWithIndex, ParameterIndex, Statement};
use uuid::Uuid;

/// Unique identifier for an asset module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetModuleID(Uuid);

impl AssetModuleID {
    /// Creates a new `AssetModuleID` with a generated UUID.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        AssetModuleID(Uuid::new_v4())
    }

    /// Creates an `AssetModuleID` from a string representation of a UUID.
    pub(crate) fn from_string(s: &str) -> Option<Self> {
        Uuid::parse_str(s).ok().map(AssetModuleID)
    }
}

impl fmt::Display for AssetModuleID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BindableWithIndex for AssetModuleID {
    fn bind<T: ParameterIndex>(
        self,
        statement: &mut Statement,
        index: T,
    ) -> Result<(), sqlite::Error> {
        let uuid = self.0.to_string();
        uuid.as_str().bind(statement, index)
    }
}

/// Represents a module in the asset database.
#[derive(Debug, Clone)]
pub struct AssetModule {
    /// Unique identifier for the module.
    pub id: AssetModuleID,

    /// Name of the module.
    pub name: String,
}
