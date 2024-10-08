#![allow(unused)]

use axum::{async_trait, extract::{FromRequestParts, State}, http::{request::Parts, Request}, middleware::Next, response::Response, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::{ctx::Ctx, model::ModelController, web::AUTH_TOKEN, Error, Result};

pub async fn mw_ctx_resolver<B> (
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>
    let result_ctx = match auth_token
    .ok_or(Error::AuthFailNoAuthTokenCookie)
    .and_then(parse_token
    ) {
       Ok((user_id, _exp, sign)) =>  {
        // TODO: Token components validations.
        Ok(Ctx::new(user_id))
       }
       Err(e) => Err(e)
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err() 
     && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) 
    {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}


pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>, 
    next: Next<B>
) -> Result<Response> {
    println!("->> {:<12} - mw_required_auth - {ctx:?}", "MIDDLEWARE");

    // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // // TODO: Real auth-token parsing & validation

    // // parse token
    // let (user_id, exp, sign) =  auth_token.ok_or(Error::AuthFailNoAuthTokenCookie).and_then(parse_token)?;

    // // TODO: Token components validations.

    ctx?;
        
    Ok(next.run(req).await)
}


// region:     -- Ctx
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;
    
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        // // User the cookies extractor.
        // let cookies = parts.extract::<Cookies>().await.unwrap();

        // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // // parse token
        // let (user_id, exp, sign) = auth_token
        // .ok_or(Error::AuthFailNoAuthTokenCookie)
        // .and_then(parse_token)?;

        // // TODO: Token components validation.

        // Ok(Ctx::new(user_id))

        parts
        .extensions
        .get::<Result<Ctx>>()
        .ok_or(Error::AuthFailCtxNotInRequestExt)?
        .clone()

    } 
}

// endregion:  -- Ctx

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // regex expression
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;
    
    let user_id: u64 = user_id
    .parse()
    .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}