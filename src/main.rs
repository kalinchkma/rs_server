

use std::net::SocketAddr;

use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;


#[allow(unused)]

#[tokio::main]
async fn main() {
    // routes
    let routes = Router::new().merge(hello_routes());

    // region: --- Start server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await.unwrap()
    // endregion: -- Start server

}




// region: -- hello routers
fn hello_routes() -> Router {
    Router::new().route("/hello/:name", get(handler_path))
    .route("/hello-world", get(hello_world))
}
// enregion: -- hello routers

// region: -- Handler hello world
/// Query paramers struct
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

async fn hello_world(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - hellow_world", "HANDLER");

    let name = params.name.as_deref().unwrap_or("world!");
    Html(format!("Hello <strong>{name}</strong>"))
}
// endregion: -- Handler hello world

// extract value from path
async fn handler_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_path - {name:?}", "HANDLER");
    Html(format!("Hello <i>{name}</i>"))

}
