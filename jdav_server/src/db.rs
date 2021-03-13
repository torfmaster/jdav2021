use base64;
use rand::prelude::*;
use serde_json::{from_reader, to_writer};
use sha2::{Digest, Sha256};
use shared::Kilometer;
use std::sync::Arc;
use tokio::fs::File;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::models::{DatabaseModel, Id, KilometerEntry, User, UserAuth};

static DATABASE_FILENAME: &'static str = "./database.json";

#[derive(Clone)]
pub struct Database {
    pub database: Arc<Mutex<DatabaseModel>>,
}

impl Database {
    pub async fn create_user(&self, new_user: UserAuth) -> bool {
        let mut db = self.database.lock().await;

        let mut salt_bytes: [u8; 8] = [0; 8];
        rand::thread_rng().fill_bytes(&mut salt_bytes);
        let salt = base64::encode(salt_bytes);

        let mut hasher = Sha256::new();
        hasher.update(new_user.pass + &salt);
        let hash = hasher.finalize();
        let hash_b64 = base64::encode(hash);

        let user = User {
            hash: hash_b64,
            salt: salt,
        };

        if !db.users.contains_key(&new_user.name) {
            println!("{} registered", &new_user.name);
            db.users.insert(new_user.name, user);
            self.save_database(&db).await;
            return true;
        }
        false
    }

    pub async fn authenticate_user(&self, user_auth: UserAuth) -> bool {
        let db = self.database.lock().await;

        if db.users.contains_key(&user_auth.name) {
            let user = db.users.get(&user_auth.name).unwrap();

            let mut hasher = Sha256::new();
            hasher.update(user_auth.pass + &user.salt);
            let hash = base64::encode(hasher.finalize());
            if &hash == &user.hash {
                return true;
            }
        }
        false
    }

    pub async fn create_kilometer_entry(&self, kilometer: Kilometer, user: String) -> Uuid {
        let mut db = self.database.lock().await;
        let new_id = Uuid::new_v4();
        let new_entry: KilometerEntry = KilometerEntry {
            id: Id { id: new_id },
            kilometers: kilometer,
        };

        let entries_for_user = db.entries.get_mut(&user);
        match entries_for_user {
            Some(entries_for_user) => {
                entries_for_user.push(new_entry);
            }
            None => {
                let mut map = Vec::new();
                map.push(new_entry);
                db.entries.insert(user, map);
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
        let entries_for_user = db.entries.get_mut(&user);
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
        let entries_for_user = db.entries.get_mut(&user);
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
        let entries_for_user = db.entries.get(&user);
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
            database: Arc::new(Mutex::new(DatabaseModel::default())),
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
