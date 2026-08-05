use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs,
    path::PathBuf,
    time::Duration,
};

use sqlx::{
    migrate::{MigrateError, Migrator},
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use tauri::{AppHandle, Manager};

const DATABASE_FILE_NAME: &str = "avoryn.db";

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[derive(Debug)]
pub enum DatabaseError {
    PathResolution,
    DirectoryCreation(std::io::Error),
    Connection(sqlx::Error),
    Migration(MigrateError),
}

impl Display for DatabaseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathResolution => {
                write!(formatter, "Unable to locate AVORYN's local data directory.")
            }
            Self::DirectoryCreation(_) => {
                write!(
                    formatter,
                    "Unable to prepare AVORYN's local data directory."
                )
            }
            Self::Connection(_) => {
                write!(formatter, "Unable to connect to the AVORYN database.")
            }
            Self::Migration(_) => {
                write!(formatter, "Unable to initialize the AVORYN database.")
            }
        }
    }
}

impl Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::PathResolution => None,
            Self::DirectoryCreation(error) => Some(error),
            Self::Connection(error) => Some(error),
            Self::Migration(error) => Some(error),
        }
    }
}

pub fn database_file_path(app: &AppHandle) -> Result<PathBuf, DatabaseError> {
    let app_data_directory = app
        .path()
        .app_local_data_dir()
        .map_err(|_| DatabaseError::PathResolution)?;

    fs::create_dir_all(&app_data_directory).map_err(DatabaseError::DirectoryCreation)?;

    Ok(app_data_directory.join(DATABASE_FILE_NAME))
}

pub async fn initialize_database(app: &AppHandle) -> Result<SqlitePool, DatabaseError> {
    let database_path = database_file_path(app)?;

    let connection_options = SqliteConnectOptions::new()
        .filename(database_path)
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
        .map_err(DatabaseError::Connection)?;

    MIGRATOR
        .run(&pool)
        .await
        .map_err(DatabaseError::Migration)?;

    Ok(pool)
}
