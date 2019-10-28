use actix_web::{HttpMessage, HttpRequest, HttpResponse, AsyncResponder, FutureResponse};
use futures::Future;

use share::common::AppState;
use model::info::RusterInfo;

pub fn ruster_info(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(RusterInfo)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msgs) => Ok(HttpResponse::Ok().json(msgs)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}
