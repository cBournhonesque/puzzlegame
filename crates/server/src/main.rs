use shared::config::Config;
use std::future::Future;
use std::pin::Pin;

#[tokio::main]
async fn main() {
    let config = Config::load();

    // route that serves up the client application (wasm + assets)
    let http_route = warp::fs::dir("dist");

    let routes = warp::get()
        .and(http_route);

    let serve = warp::serve(routes);

    // don't use tls if dev
    let serve: Pin<Box<dyn Future<Output=()>>> = if cfg!(debug_assertions) {
        warn!("starting server in dev mode without TLS");
        Box::pin(serve.run(([0, 0, 0, 0], config.port)))
    }
}