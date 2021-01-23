use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::models::{DatabaseModel, Id, Kilometer, KilometerEntry};
#[derive(Clone)]
pub struct Database {
    pub database: Arc<Mutex<DatabaseModel>>,
}

impl Database {
    pub async fn create_kilometer_entry(&self, kilometer: Kilometer, user: String) -> Uuid {
        let mut db = self.database.lock().await;
        let new_id = Uuid::new_v4();
        let new_entry: KilometerEntry = KilometerEntry {
            id: Id { id: new_id },
            kilometers: kilometer,
        };

        let entries_for_user = db.get_mut(&user);
        match entries_for_user {
            Some(entries_for_user) => {
                entries_for_user.push(new_entry);
            }
            None => {
                let mut map = Vec::new();
                map.push(new_entry);
                db.insert(user, map);
            }
        }

        new_id
    }
    pub async fn retrieve_kilometer_entry(
        &self,
        ident: Id,
        user: String,
    ) -> Option<KilometerEntry> {
        let mut db = self.database.lock().await;
        let entries_for_user = db.get_mut(&user);
        match entries_for_user {
            Some(entries_for_user) => {
                for entry in entries_for_user.iter() {
                    if entry.id == ident {
                        return Some(entry.clone());
                    }
                }
                return None;
            }
            None => {
                return None;
            }
        }
    }
    pub async fn retrieve_kilometer_all(&self, user: String) -> Option<Vec<KilometerEntry>> {
        let mut db = self.database.lock().await;
        let entries_for_user = db.get_mut(&user);
        match entries_for_user {
            Some(entries_for_user) => {
                return Some(entries_for_user.clone());
            }
            None => {
                return None;
            }
        }
    }
    pub async fn retrieve_kilometer_sum(&self, user: String) -> Option<Kilometer> {
        let db = self.database.lock().await;
        let entries_for_user = db.get(&user);
        match entries_for_user {
            Some(entries_for_user) => {
                let mut sum: f32 = 0.0;
                for entry in entries_for_user.iter() {
                    sum += entry.kilometers.kilometers;
                }
                return Some(Kilometer { kilometers: sum });
            }
            None => {
                return None;
            }
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Database {
            database: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub fn init_db() -> Database {
    Database::default()
}
