use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Database;
use crate::handlers;
use crate::models::{Id, Kilometer};

pub fn routes(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let static_content = warp::get().and(warp::fs::dir("../jdav_client/dist"));

    static_content
        .or(create_entry(db.clone()))
        .or(retrieve_entry(db.clone()))
        .or(retrieve_all(db.clone()))
        .or(retrieve_sum(db.clone()))
}

fn create_entry(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("distanz" / String / "laufen")
        .and(warp::put())
        .and(json_kilometer_entry())
        .and(with_database(db.clone()))
        .and_then(handlers::create_kilometer_entry)
}

fn json_kilometer_entry() -> impl Filter<Extract = (Kilometer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn retrieve_entry(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("distanz" / String / "laufen")
        .and(json_kilometer_retrieve())
        .and(with_database(db.clone()))
        .and_then(handlers::retrieve_kilometer_entry)
}

fn json_kilometer_retrieve() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn retrieve_all(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("distanz" / String / "laufen" / "all")
        .and(warp::path::end())
        .and(with_database(db.clone()))
        .and_then(handlers::retrieve_kilometer_all)
}

fn retrieve_sum(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("distanz" / String / "laufen" / "sum")
        .and(with_database(db.clone()))
        .and_then(handlers::retrieve_kilometer_sum)
}

fn with_database(
    database: Database,
) -> impl Filter<Extract = (Database,), Error = Infallible> + Clone {
    warp::any().map(move || database.clone())
}
