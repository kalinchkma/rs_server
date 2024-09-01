

use std::net::SocketAddr;

use axum::{extract::Query, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;


#[allow(unused)]

#[tokio::main]
async fn main() {
    // hello world routes
    let route_hello_world = Router::new().route("/hello-world", get(hello_world)); 

    // region: --- Start server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(route_hello_world.into_make_service())
        .await.unwrap()
    // endregion: -- Start server

}

// region: -- Handler hello world
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
