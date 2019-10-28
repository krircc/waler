use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::Future;

use model::user::{AdminUsers};
use model::theme::{AdminThemes};
use share::common::AppState;

pub fn admin_users(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(AdminUsers)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msgs) => Ok(HttpResponse::Ok().json(msgs)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn admin_themes(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(AdminThemes)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msgs) => Ok(HttpResponse::Ok().json(msgs)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}