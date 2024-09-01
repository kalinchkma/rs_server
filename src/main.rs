#[allow(unused)]

// export error and result to use for other
pub use self::error::{Error, Result};

use std::net::SocketAddr;
use axum::{extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router};
use model::ModelController;
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialized ModelController
    let mc = ModelController::new().await?;    

    // routes apis
    // route_layer used because we want to only apply this middleware for this routes
    let routes_apis = web::routes_tickes::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    
    // routes
    let routes = Router::new()
    .merge(hello_routes())
    .merge(web::routes_login::routes())
    .nest("/api", routes_apis)
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static()); // fallback_service use for static file routing

    // region: --- Start server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await.unwrap();
    // endregion: -- Start server


    Ok(())
}

// region: -- middleware
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main middleware response mapper", "RES_MAPPER");
    println!();
    res
}
// endregion: -- middleware

// region: --static file routing
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
// endregion: -- static file routing


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
