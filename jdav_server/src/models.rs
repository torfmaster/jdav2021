use serde::{Deserialize, Serialize};
use shared::KilometerEntry;
use std::collections::HashMap;

use crate::db::migration::DatabaseVersion;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub hash: String,
    pub salt: String,
}

pub type UserKey = String;
pub type EntryDatabaseModel = HashMap<UserKey, Vec<KilometerEntry>>;
pub type UserDatabaseModel = HashMap<UserKey, User>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseModel {
    pub entries: EntryDatabaseModel,
    pub users: UserDatabaseModel,
    pub database_version: DatabaseVersion,
}

impl Default for DatabaseModel {
    fn default() -> Self {
        DatabaseModel {
            entries: HashMap::new(),
            users: HashMap::new(),
            database_version: DatabaseVersion::V1,
        }
    }
}
