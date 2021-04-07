use serde::{Deserialize, Serialize};
use shared::KilometerEntry;
use std::collections::HashMap;

use super::DatabaseVersion;

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
}

impl Default for DatabaseModel {
    fn default() -> Self {
        DatabaseModel {
            entries: HashMap::new(),
            users: HashMap::new(),
        }
    }
}

impl DatabaseModel {
    pub fn to_v1(self) -> crate::models::DatabaseModel {
        crate::models::DatabaseModel {
            entries: self
                .entries
                .iter()
                .map(|(user, entry)| (user.to_owned(), map_entry(entry)))
                .collect::<HashMap<_, _>>(),
            users: map_users(self.users),
            database_version: DatabaseVersion::V1,
        }
    }
}

fn map_entry(entry: &[KilometerEntry]) -> Vec<shared::KilometerEntry> {
    entry
        .iter()
        .map(|entry| shared::KilometerEntry {
            id: shared::Id { id: entry.id.id },
            kilometers: shared::Kilometer {
                kilometers: entry.kilometers.kilometers,
            },
            kind: shared::Kind::Running,
        })
        .collect::<Vec<_>>()
}

fn map_users(
    users: HashMap<UserKey, User>,
) -> HashMap<crate::models::UserKey, crate::models::User> {
    users
        .iter()
        .map(|(user_key, user)| {
            (
                user_key.to_owned(),
                crate::models::User {
                    hash: user.hash.clone(),
                    salt: user.salt.clone(),
                },
            )
        })
        .collect::<HashMap<_, _>>()
}
