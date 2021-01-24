mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let database = db::init_db();

    warp::serve(routes::routes(database.clone()))
        .run(([0, 0, 0, 0], 8080))
        .await;
}
