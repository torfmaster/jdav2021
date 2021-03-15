use serde::Deserialize;

use self::migration::DatabaseVersion;

pub mod db;
pub mod migration;

#[derive(Deserialize, Debug)]
pub struct DatabaseWithVersion {
    database_version: Option<DatabaseVersion>,
}
