mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let database = db::init_db().await;

    warp::serve(routes::routes(database.clone()))
        .run(([127, 0, 0, 1], 8080))
        .await;
}
