#[allow(unused)]

// export error and result to use for other
pub use self::error::{Error, Result};

use std::net::SocketAddr;
use axum::{extract::{Path, Query}, http::{uri, Method, Uri}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Json, Router};
use ctx::Ctx;
use log::log_request;
use model::ModelController;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
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
    .layer(middleware::from_fn_with_state(
        mc.clone(),
        web::mw_auth::mw_ctx_resolver,
    ))
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
async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response
) -> Response {
    println!("->> {:<12} - main middleware response mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error
    let service_error = res.extensions().get::<Error>();

    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new response
    let error_reponse = client_status_error
    .as_ref().map(|&(ref status_code, ref client_error)| {
        let client_error_body = json!({
            "error": {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string()
            }
        });

        println!("    ->> client_error_body: {client_error_body}");

        // Build the new response from the client_error_body
        (*status_code, Json(client_error_body)).into_response()
    });

    // Build and log the server log line.
    // println!(" ->> server log line - {uuid} - Error: {service_error:?}");
    // 
    let client_error = client_status_error.unzip().1; 
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_reponse.unwrap_or(res)
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
