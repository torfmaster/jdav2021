mod db;
mod handlers;
mod middleware;
mod models;
mod routes;

use structopt::StructOpt;
use tokio::sync::oneshot::Sender;

use tokio::sync::oneshot;

#[derive(StructOpt, Debug)]
#[structopt(name = "jdav_server")]
struct Options {
    #[structopt(short, long)]
    use_tls: bool,
}

#[derive(Debug)]
struct ShutDownCommand;

#[tokio::main]
async fn main() {
    let database = db::db::init_db().await;

    let opt = Options::from_args();

    let (shutdown_trigger, shutdown_command) = oneshot::channel::<ShutDownCommand>();

    handle_shutdown(shutdown_trigger);

    let signal_handler = async {
        shutdown_command.await.ok();
        println!("Gracefully shutting down");
    };

    if opt.use_tls {
        warp::serve(routes::routes(database.clone()))
            .tls()
            .cert_path("/etc/letsencrypt/live/kebes.dnshome.de/fullchain.pem")
            .key_path("/etc/letsencrypt/live/kebes.dnshome.de/privkey.pem")
            .bind_with_graceful_shutdown(([0, 0, 0, 0], 8443), signal_handler)
            .1
            .await;
    } else {
        warp::serve(routes::routes(database.clone()))
            .bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), signal_handler)
            .1
            .await;
    };
}

fn handle_shutdown(tx: Sender<ShutDownCommand>) {
    use tokio::signal::unix::{signal, SignalKind};

    let term_receiver = async {
        let mut stream = signal(SignalKind::terminate()).unwrap();
        stream.recv().await;
        tx.send(ShutDownCommand).unwrap();
    };

    tokio::spawn(term_receiver);
}
