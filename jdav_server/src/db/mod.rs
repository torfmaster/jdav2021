use serde::{Deserialize, Serialize};

pub mod db;
pub mod migration_to_v1;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum DatabaseVersion {
    V1,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseWithVersion {
    database_version: Option<DatabaseVersion>,
}
