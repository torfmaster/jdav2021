use std::convert::Infallible;

use shared::{Kilometer, UserAuth};
use warp::{self, http::StatusCode};

use crate::{db::Database, middleware::extract_basicauth};

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
    if database.authenticate_user(&user_auth).await {
        Ok(Box::new(warp::reply::json(&"user auth'd")))
    } else {
        Ok(Box::new(warp::reply::with_status(
            "Wrong user/password".to_owned(),
            StatusCode::UNAUTHORIZED,
        )))
    }
}

pub async fn create_kilometer_entry(
    user: String,
    header: String,
    kilometer: Kilometer,
    database: Database,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let authorization = authorize(&user, header, database.clone()).await;

    if authorization.is_ok() {
        let id = database.create_kilometer_entry(kilometer, user).await;
        Ok(Box::new(warp::reply::json(&id.to_string())))
    } else {
        Ok(Box::new(warp::reply::with_status(
            format!("Unauthorized"),
            StatusCode::UNAUTHORIZED,
        )))
    }
}

pub async fn authorize(user: &String, header: String, database: Database) -> Result<(), ()> {
    let auth = extract_basicauth(header).map_err(|_| ())?;

    if !database.authenticate_user(&auth).await || &auth.name != user {
        return Err(());
    }
    Ok(())
}
