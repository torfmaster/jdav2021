use serde_json::{from_reader, to_writer};
use std::{collections::HashMap, sync::Arc};
use tokio::fs::File;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::models::{DatabaseModel, Id, Kilometer, KilometerEntry};

static DATABASE_FILENAME: &'static str = "./database.json";

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
        self.save_database(&db).await;
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
    async fn save_database(&self, db: &DatabaseModel) {
        //let db = self.database.lock().await;
        let file = File::create(DATABASE_FILENAME).await;
        match file {
            Ok(json) => {
                to_writer(json.into_std().await, &db.clone()).expect("error writing to file");
            }
            Err(_) => {}
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

pub async fn init_db() -> Database {
    let file = File::open(DATABASE_FILENAME).await;

    match file {
        Ok(file) => {
            let data = from_reader(file.into_std().await).unwrap();
            return Database {
                database: Arc::new(Mutex::new(data)),
            };
        }
        Err(_) => {
            return Database::default();
        }
    }
}
