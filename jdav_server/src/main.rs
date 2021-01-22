use serde::{Deserialize, Serialize};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{http, Filter};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Kilometer {
    pub kilometers: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Id {
    pub id: usize,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct KilometerEntry {
    pub id: Id,
    pub kilometers: Kilometer,
}

type Database = Arc<Mutex<Vec<KilometerEntry>>>;

#[tokio::main]
async fn main() {
    let database = Arc::new(Mutex::new(Vec::<KilometerEntry>::new()));
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .and(with_database(database.clone()))
        .and_then(responder);

    let create_entry = warp::put()
        .and(warp::path("distanz"))
        .and(warp::path("laufen"))
        .and(warp::path::end())
        .and(json_kilometer_entry())
        .and(with_database(database.clone()))
        .and_then(create_kilometer_entry);

    let retrieve_entry = warp::get()
        .and(warp::path("distanz"))
        .and(warp::path("laufen"))
        .and(warp::path::end())
        .and(json_kilometer_retrieve())
        .and(with_database(database.clone()))
        .and_then(retrieve_kilometer_entry);

    let retrieve_all = warp::get()
        .and(warp::path("distanz"))
        .and(warp::path("laufen"))
        .and(warp::path::end())
        .and(with_database(database.clone()))
        .and_then(retrieve_kilometer_all);

    let retrieve_sum = warp::get()
        .and(warp::path("distanz"))
        .and(warp::path("laufen"))
        .and(warp::path("sum"))
        .and(warp::path::end())
        .and(with_database(database.clone()))
        .and_then(retrieve_kilometer_sum);

    let routes = hello
        .or(create_entry)
        .or(retrieve_entry)
        .or(retrieve_all)
        .or(retrieve_sum);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn create_kilometer_entry(
    kilometer: Kilometer,
    database: Database,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut db = database.lock().await;
    let db_length: usize = db.len();
    let entry: KilometerEntry = KilometerEntry {
        id: Id { id: db_length },
        kilometers: kilometer,
    };
    db.push(entry);

    Ok(warp::reply::with_status(
        db_length.to_string(),
        http::StatusCode::CREATED,
    ))
}

fn json_kilometer_entry() -> impl Filter<Extract = (Kilometer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn retrieve_kilometer_entry(
    ident: Id,
    database: Database,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let db = database.lock().await;
    if ident.id < db.len() {
        let entry = db.get(ident.id);
        Ok(Box::new(warp::reply::json(&entry)))
    } else {
        Ok(Box::new(warp::reply::with_status(
            format!("no entry with id {}", ident.id),
            http::StatusCode::NOT_FOUND,
        )))
    }
}

fn json_kilometer_retrieve() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn retrieve_kilometer_all(database: Database) -> Result<impl warp::Reply, warp::Rejection> {
    let db = database.lock().await;

    Ok(warp::reply::json(&db.to_vec()))
}

async fn retrieve_kilometer_sum(database: Database) -> Result<impl warp::Reply, warp::Rejection> {
    let db = database.lock().await;
    let mut sum: f32 = 0.0;
    for i in db.to_vec() {
        sum += i.kilometers.kilometers;
    }
    Ok(warp::reply::json(&sum))
}

fn with_database(
    database: Database,
) -> impl Filter<Extract = (Database,), Error = Infallible> + Clone {
    warp::any().map(move || database.clone())
}

async fn responder(name: String, database: Database) -> Result<impl warp::Reply, Infallible> {
    let wurst = database.lock().await;
    println!("{:?}", *wurst);

    Ok(format!("Hello, {}!", name))
}
