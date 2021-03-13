use std::convert::Infallible;

use shared::Kilometer;
use warp::{self, http::StatusCode};

use crate::db::Database;
use crate::models::{Id, KilometerEntry, UserAuth};

pub async fn create_user(
    new_user: UserAuth,
    database: Database,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    if database.create_user(new_user).await {
        Ok(Box::new(warp::reply::json(&"user created")))
    } else {
        Ok(Box::new(warp::reply::with_status(
            "Error Creating User".to_owned(),
            StatusCode::FORBIDDEN,
        )))
    }
}

pub async fn authenticate_user(
    user_auth: UserAuth,
    database: Database,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    if database.authenticate_user(user_auth).await {
        Ok(Box::new(warp::reply::json(&"user auth'd")))
    } else {
        Ok(Box::new(warp::reply::with_status(
            "Wrong user/password".to_owned(),
            StatusCode::UNAUTHORIZED,
        )))
    }
}

pub async fn create_kilometer_entry(
    _user: String,
    pass: String,
    kilometer: Kilometer,
    database: Database,
) -> Result<impl warp::Reply, Infallible> {
    let id = database.create_kilometer_entry(kilometer, _user).await;

    println!("{}", pass);

    Ok(warp::reply::json(&id.to_string()))
}

pub async fn retrieve_kilometer_entry(
    user: String,
    pass: String,
    ident: Id,
    database: Database,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let entry: Option<KilometerEntry> = database.retrieve_kilometer_entry(ident, user).await;
    match entry {
        Some(entry) => {
            return Ok(Box::new(warp::reply::json(&entry)));
        }
        None => {
            return Ok(Box::new(warp::reply::with_status(
                format!("no entry with id {}", ident.id),
                StatusCode::NOT_FOUND,
            )))
        }
    }
}

pub async fn retrieve_kilometer_all(
    user: String,
    pass: String,
    database: Database,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let entries = database.retrieve_kilometer_all(user.clone()).await;
    match entries {
        Some(entries) => Ok(Box::new(warp::reply::json(&entries))),
        None => Ok(Box::new(warp::reply::with_status(
            format!("no user found with id {}", user),
            StatusCode::NOT_FOUND,
        ))),
    }
}

pub async fn retrieve_kilometer_sum(
    user: String,
    pass: String,
    database: Database,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let sum: Option<Kilometer> = database.retrieve_kilometer_sum(user.clone()).await;
    match sum {
        Some(sum) => {
            return Ok(Box::new(warp::reply::json(&sum)));
        }
        None => Ok(Box::new(warp::reply::with_status(
            format!("user {} not found", user),
            StatusCode::NOT_FOUND,
        ))),
    }
}
