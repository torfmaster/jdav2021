use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub struct Kilometer {
    pub kilometers: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct Id {
    pub id: Uuid,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KilometerEntry {
    pub id: Id,
    pub kilometers: Kilometer,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub name: Id,
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
