mod db;
mod handlers;
mod models;
mod routes;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jdav_server")]
struct Options {
    #[structopt(short, long)]
    use_tls: bool,
}

#[tokio::main]
async fn main() {
    let database = db::init_db().await;

    let opt = Options::from_args();
    if opt.use_tls {
        warp::serve(routes::routes(database.clone()))
            .tls()
            .cert_path("/etc/letsencrypt/live/kebes.dnshome.de/fullchain.pem")
            .key_path("/etc/letsencrypt/live/kebes.dnshome.de/privkey.pem")
            .run(([0, 0, 0, 0], 443))
            .await;
    } else {
        warp::serve(routes::routes(database.clone()))
            .run(([0, 0, 0, 0], 8080))
            .await;
    }
}
