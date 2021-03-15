use rand::prelude::*;
use serde_json::to_writer;
use sha2::{Digest, Sha256};
use shared::{Highscore, HighscoreEntry, Kilometer, UserAuth};
use std::sync::Arc;
use tokio::fs::File;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::models::{DatabaseModel, Id, KilometerEntry, User};

pub static DATABASE_FILENAME: &str = "./database.json";

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
            salt,
        };

        if !db.users.contains_key(&new_user.name) {
            db.users.insert(new_user.name, user);
            self.save_database(&db).await;
            return true;
        }
        false
    }

    pub async fn authenticate_user(&self, user_auth: &UserAuth) -> bool {
        let db = self.database.lock().await;

        if db.users.contains_key(&user_auth.name) {
            let user = db.users.get(&user_auth.name).unwrap();

            let mut hasher = Sha256::new();
            hasher.update(user_auth.pass.clone() + &user.salt);
            let hash = base64::encode(hasher.finalize());
            if hash == user.hash {
                return true;
            }
        }
        false
    }

    pub async fn create_kilometer_entry(
        &self,
        kilometer: Kilometer,
        user: String,
        kind: crate::models::Kind,
    ) -> Uuid {
        let mut db = self.database.lock().await;
        let new_id = Uuid::new_v4();
        let new_entry: KilometerEntry = KilometerEntry {
            id: Id { id: new_id },
            kilometers: kilometer,
            kind,
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

    async fn save_database(&self, db: &DatabaseModel) {
        let file = File::create(DATABASE_FILENAME).await;
        if let Ok(json) = file {
            to_writer(json.into_std().await, &db.clone()).expect("error writing to file");
        }
    }

    pub async fn get_highscore(&self) -> Highscore {
        let db = self.database.lock().await;
        get_highscore(&db)
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
    crate::db::migration::migrate().await.unwrap_or_default()
}

fn get_highscore(database: &DatabaseModel) -> Highscore {
    let mut list = database
        .entries
        .iter()
        .map(|(key, value)| HighscoreEntry {
            user: key.clone(),
            points: value.iter().fold(0.0, |acc, entry: &KilometerEntry| {
                acc + entry.kilometers.kilometers
            }),
        })
        .collect::<Vec<_>>();
    list.sort_by(|entry1, entry2| entry2.points.partial_cmp(&entry1.points).unwrap());
    Highscore { list }
}

#[cfg(test)]
mod test {
    use shared::Kilometer;
    use uuid::Uuid;

    use super::get_highscore;
    use crate::models::{DatabaseModel, Id, KilometerEntry};

    #[test]
    pub fn can_process_one_kilometer_entry() {
        let mut database: DatabaseModel = Default::default();
        let id1 = Id { id: Uuid::new_v4() };
        let id2 = Id { id: Uuid::new_v4() };

        let kilometer1 = Kilometer { kilometers: 2.0 };
        let kilometer2 = Kilometer { kilometers: 1.0 };

        let kilometer_entry = KilometerEntry {
            id: id1,
            kilometers: kilometer1,
            kind: crate::models::Kind::Running,
        };

        let kilometer_entry2 = KilometerEntry {
            id: id2,
            kilometers: kilometer2,
            kind: crate::models::Kind::Running,
        };
        database
            .entries
            .insert("user1".to_owned(), vec![kilometer_entry]);

        database
            .entries
            .insert("user2".to_owned(), vec![kilometer_entry2]);

        let score = get_highscore(&database);
        let first = score.list.get(0).unwrap();
        let second = score.list.get(1).unwrap();
        assert_eq!(first.user, "user1");
        assert_eq!(second.user, "user2");
    }
}
