use std::convert::Infallible;

use warp::{self, http::StatusCode, Reply};

use crate::db::Database;
use crate::models::{Id, Kilometer, KilometerEntry, UserAuth};

pub async fn create_user(
    new_user: UserAuth,
    database: Database,
) -> Result<impl warp::Reply, Infallible> {
    if database.create_user(new_user).await {
        Ok(warp::reply::with_status(
            "user created",
            StatusCode::CREATED,
        ))
    } else {
        Ok(warp::reply::with_status(
            "error creating user",
            StatusCode::CREATED,
        ))
    }
}

pub async fn authenticate_user(
    user_auth: UserAuth,
    database: Database,
) -> Result<impl warp::Reply, Infallible> {
    if database.authenticate_user(user_auth).await {
        Ok(warp::reply::with_status("user auth'd", StatusCode::CREATED))
    } else {
        Ok(warp::reply::with_status(
            "user not auth'd - wrong name or password",
            StatusCode::CREATED,
        ))
    }
}

pub async fn create_kilometer_entry(
    _user: String,
    pass: String,
    kilometer: Kilometer,
    database: Database,
) -> Result<impl warp::Reply, Infallible> {
    let id = database.create_kilometer_entry(kilometer, _user).await;

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
    let sum = database.retrieve_kilometer_sum(user.clone()).await;
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
