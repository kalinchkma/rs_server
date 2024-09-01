use axum::{http::StatusCode, response::IntoResponse};


// export type alias
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LogginFail,

    // -- Auth errors
    AuthFailNoAuthTokenCookie,

    // -- Model errors
    TicketDeleteFailIdNotFound {id: u64}
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}