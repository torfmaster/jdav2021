use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::{http, Filter};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Kilometer {
    pub kilometers: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct Id {
    pub id: Uuid,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct KilometerEntry {
    pub id: Id,
    pub kilometers: Kilometer,
}

type UserKey = String;
type DatabaseModel = HashMap<UserKey, Vec<KilometerEntry>>;

#[derive(Clone)]
struct Database {
    pub database: Arc<Mutex<DatabaseModel>>,
}

impl Database {
    async fn create_kilometer_entry(&self, kilometer: Kilometer, user: String) -> Uuid {
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
}

impl Default for Database {
    fn default() -> Self {
        Database {
            database: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tokio::main]
async fn main() {
    let database: Database = Default::default();

    let create_entry = warp::put()
        .and(warp::path!("distanz" / String / "laufen"))
        .and(json_kilometer_entry())
        .and(with_database(database.clone()))
        .and_then(create_kilometer_entry);

    let retrieve_entry = warp::get()
        .and(warp::path!("distanz" / String / "laufen"))
        .and(json_kilometer_retrieve())
        .and(with_database(database.clone()))
        .and_then(retrieve_kilometer_entry);

    let retrieve_all = warp::get()
        .and(warp::path!("distanz" / String / "laufen"))
        .and(warp::path::end())
        .and(with_database(database.clone()))
        .and_then(retrieve_kilometer_all);

    let retrieve_sum = warp::get()
        .and(warp::path!("distanz" / String / "laufen" / "sum"))
        .and(with_database(database.clone()))
        .and_then(retrieve_kilometer_sum);

    let routes = 
        create_entry
        .or(retrieve_entry)
        .or(retrieve_all)
        .or(retrieve_sum);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn create_kilometer_entry(
    _user: String,
    kilometer: Kilometer,
    database: Database,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut db = database.database.lock().await;
    let new_id = Uuid::new_v4();
    let new_entry: KilometerEntry = KilometerEntry {
        id: Id { id: new_id },
        kilometers: kilometer,
    };

    let entries_for_user = db.get_mut(&_user);
    match entries_for_user {
        Some(entries_for_user) => {
            entries_for_user.push(new_entry);
        }
        None => {
            let mut map = Vec::new();
            map.push(new_entry);
            db.insert(_user, map);
        }
    }

    Ok(warp::reply::with_status(
        new_id.to_string(),
        http::StatusCode::CREATED,
    ))
}

fn json_kilometer_entry() -> impl Filter<Extract = (Kilometer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn retrieve_kilometer_entry(
    user: String,
    ident: Id,
    database: Database,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let mut db = database.database.lock().await;
    let entries_for_user = db.get_mut(&user);
    match entries_for_user {
        Some(entries_for_user) => {
            for entry in entries_for_user.iter() {
                if entry.id == ident {
                    return Ok(Box::new(warp::reply::json(&entry)));
                }
            }
            Ok(Box::new(warp::reply::with_status(
                format!("no entry with id {}", ident.id),
                http::StatusCode::NOT_FOUND,
            )))
        }
        None => Ok(Box::new(warp::reply::with_status(
            format!("no user found with id {}", user),
            http::StatusCode::NOT_FOUND,
        ))),
    }
}

fn json_kilometer_retrieve() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn retrieve_kilometer_all(
    _user_id: String,
    database: Database,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let db = database.database.lock().await;
    let entry_per_user = db.get(&_user_id);
    if let Some(entry_per_user) = entry_per_user {
        Ok(Box::new(warp::reply::json(entry_per_user)))
    } else {
        Ok(Box::new(warp::reply::with_status(
            format!("no user found with id {}", _user_id),
            http::StatusCode::NOT_FOUND,
        )))
    }
}

async fn retrieve_kilometer_sum(
    user: String,
    database: Database,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let db = database.database.lock().await;
    let mut sum: f32 = 0.0;
    let entries_for_user = db.get(&user);
    match entries_for_user {
        Some(entries_for_user) => {
            for entry in entries_for_user.iter() {
                sum += entry.kilometers.kilometers;
            }
            Ok(Box::new(warp::reply::json(&sum)))
        }
        None => Ok(Box::new(warp::reply::with_status(
            format!("user {} not found", user),
            http::StatusCode::NOT_FOUND,
        ))),
    }
}

fn with_database(
    database: Database,
) -> impl Filter<Extract = (Database,), Error = Infallible> + Clone {
    warp::any().map(move || database.clone())
}
