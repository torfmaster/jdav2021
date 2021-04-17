use shared::{Kilometer, Kind, UserAuth};
use warp::{self, Filter};

use crate::db::db::Database;
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
        .or(create_running_entry(db.clone()))
        .or(create_biking_entry(db.clone()))
        .or(create_climbing_entry(db.clone()))
        .or(create_swimming_entry(db.clone()))
        .or(create_skating_entry(db.clone()))
        .or(create_hiking_entry(db.clone()))
        .or(edit_kilometer_entry(db.clone()))
        .or(get_entries_for_user(db.clone()))
        .or(get_highscore(db))
}

fn create_user(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("createuser")
        .and(json_auth_user())
        .and(with_database(db))
        .and_then(handlers::create_user)
}

fn authenticate_user(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("authenticate")
        .and(json_auth_user())
        .and(with_database(db))
        .and_then(handlers::authenticate_user)
}

fn json_auth_user() -> impl Filter<Extract = (UserAuth,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn create_running_entry(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("distanz" / String / "laufen")
        .and(warp::put())
        .and(warp::any().map(|| Kind::Running))
        .and(authentication_middleware())
        .and(json_kilometer_entry())
        .and(with_database(db))
        .and_then(handlers::create_kilometer_entry)
}

fn create_biking_entry(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("distanz" / String / "radfahren")
        .and(warp::put())
        .and(warp::any().map(|| Kind::Biking))
        .and(authentication_middleware())
        .and(json_kilometer_entry())
        .and(with_database(db))
        .and_then(handlers::create_kilometer_entry)
}

fn create_climbing_entry(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("distanz" / String / "klettern")
        .and(warp::put())
        .and(warp::any().map(|| Kind::Climbing))
        .and(authentication_middleware())
        .and(json_kilometer_entry())
        .and(with_database(db))
        .and_then(handlers::create_kilometer_entry)
}

fn create_swimming_entry(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("distanz" / String / "schwimmen")
        .and(warp::put())
        .and(warp::any().map(|| Kind::Swimming))
        .and(authentication_middleware())
        .and(json_kilometer_entry())
        .and(with_database(db))
        .and_then(handlers::create_kilometer_entry)
}

fn create_skating_entry(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("distanz" / String / "skaten")
        .and(warp::put())
        .and(warp::any().map(|| Kind::Skating))
        .and(authentication_middleware())
        .and(json_kilometer_entry())
        .and(with_database(db))
        .and_then(handlers::create_kilometer_entry)
}

fn create_hiking_entry(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("distanz" / String / "wandern")
        .and(warp::put())
        .and(warp::any().map(|| Kind::Hiking))
        .and(authentication_middleware())
        .and(json_kilometer_entry())
        .and(with_database(db))
        .and_then(handlers::create_kilometer_entry)
}

fn json_kilometer_entry() -> impl Filter<Extract = (Kilometer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn edit_kilometer_entry(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("edit" / String)
        .and(warp::put())
        .and(authentication_middleware())
        .and(json_edit_kilometer_entry())
        .and(with_database(db))
        .and_then(handlers::edit_kilometer_entry)
}

fn json_edit_kilometer_entry(
) -> impl Filter<Extract = (shared::KilometerEntry,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn get_entries_for_user(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("entries" / String)
        .and(warp::get())
        .and(authentication_middleware())
        .and(with_database(db))
        .and_then(handlers::get_entries_for_user)
}

fn get_highscore(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("highscore")
        .and(warp::put())
        .and(authentication_middleware())
        .and(with_database(db))
        .and_then(handlers::get_highscore)
}
