//! This module handles the implementation of the database connection for
//! accessing game files.

use std::path::Path;

use sqlite::{Connection, ConnectionThreadSafe, Error, State, Value};

/// Database struct that encapsulates the SQLite connection.
pub struct Database {
    /// The SQLite connection to the game database.
    connection: ConnectionThreadSafe,
}

impl Database {
    /// Creates a new `Database` instance by opening a connection to the
    /// sqlite database file containing the game data.
    pub fn new(project_folder: &Path) -> Result<Self, Error> {
        let path = project_folder.join("game.awgen");
        let connection = Connection::open_thread_safe(path)?;
        let db = Database { connection };
        db.init()?;
        Ok(db)
    }

    /// Initializes the database by creating necessary tables and indices.
    fn init(&self) -> Result<(), Error> {
        self.connection.execute(
            "
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT
            );
            ",
        )?;

        Ok(())
    }

    /// Gets the value of a setting by its key.
    ///
    /// Returns `Ok(Some(value))` if the key exists, `Ok(None)` if it does not,
    /// and `Err` if there was an error querying the database.
    pub fn get_setting(&self, key: &str) -> Result<Option<String>, Error> {
        let query = "SELECT value FROM settings WHERE key = :key";
        let mut statement = self.connection.prepare(query)?;
        statement.bind((":key", key))?;

        if let State::Row = statement.next()? {
            Ok(statement.read::<String, _>("value").ok())
        } else {
            Ok(None)
        }
    }

    /// Sets a setting in the database.
    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), Error> {
        let query = "INSERT OR REPLACE INTO settings (key, value) VALUES (:key, :value)";
        let mut statement = self.connection.prepare(query)?;
        statement.bind::<&[(_, Value)]>(&[(":key", key.into()), (":value", value.into())])?;
        statement.next()?;
        Ok(())
    }

    /// Clears a setting from the database by its key.
    pub fn clear_setting(&self, key: &str) -> Result<(), Error> {
        let query = "DELETE FROM settings WHERE key = :key";
        let mut statement = self.connection.prepare(query)?;
        statement.bind((":key", key))?;
        statement.next()?;
        Ok(())
    }
}
