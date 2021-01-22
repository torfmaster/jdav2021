use std::convert::Infallible;

use warp::{self, http::StatusCode};

use crate::db::Database;
use crate::models::{Id, Kilometer};

pub async fn create_kilometer_entry(
    _user: String,
    kilometer: Kilometer,
    database: Database,
) -> Result<impl warp::Reply, Infallible> {
    let id = database.create_kilometer_entry(kilometer, _user).await;

    Ok(warp::reply::with_status(
        id.to_string(),
        StatusCode::CREATED,
    ))
}

pub async fn retrieve_kilometer_entry(
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
                StatusCode::NOT_FOUND,
            )))
        }
        None => Ok(Box::new(warp::reply::with_status(
            format!("no user found with id {}", user),
            StatusCode::NOT_FOUND,
        ))),
    }
}

pub async fn retrieve_kilometer_all(
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
            StatusCode::NOT_FOUND,
        )))
    }
}

pub async fn retrieve_kilometer_sum(
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
            StatusCode::NOT_FOUND,
        ))),
    }
}
