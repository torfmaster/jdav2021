use shared::{Kilometer, UserAuth};
use warp::{self, Filter};

use crate::db::Database;
use crate::{handlers, middleware::authentication_middleware, middleware::with_database};

pub fn routes(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let static_content = warp::get().and(warp::fs::dir("../jdav_client/dist"));
    let static_content_deployed = warp::get().and(warp::fs::dir("./dist"));

    static_content_deployed
        .or(static_content)
        .or(create_user(db.clone()))
        .or(authenticate_user(db.clone()))
        .or(create_entry(db.clone()))
}

fn create_user(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("createuser")
        .and(json_auth_user())
        .and(with_database(db.clone()))
        .and_then(handlers::create_user)
}

fn authenticate_user(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("authenticate")
        .and(json_auth_user())
        .and(with_database(db.clone()))
        .and_then(handlers::authenticate_user)
}

fn json_auth_user() -> impl Filter<Extract = (UserAuth,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn create_entry(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("distanz" / String / "laufen")
        .and(warp::put())
        .and(authentication_middleware())
        .and(json_kilometer_entry())
        .and(with_database(db.clone()))
        .and_then(handlers::create_kilometer_entry)
}

fn json_kilometer_entry() -> impl Filter<Extract = (Kilometer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
