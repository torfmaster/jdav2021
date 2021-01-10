use std::{convert::Infallible, sync::Arc};

use tokio::sync::Mutex;
use warp::Filter;

#[derive(Debug)]
struct KilometerEntry {
    pub kilometers: usize,
}

type Database = Arc<Mutex<Vec<KilometerEntry>>>;

#[tokio::main]
async fn main() {
    let database = Arc::new(Mutex::new(Vec::<KilometerEntry>::new()));
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .and(with_database(database.clone()))
        .and_then(responder);

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
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
