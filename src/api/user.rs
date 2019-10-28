use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::future::{result as FutureResult, Future};
use jwt::{decode, Header, Algorithm, Validation};

use model::user::{UserInfo,UserId, UserDelete, UserUpdate, UserUpdateImg, UserThemes,UserComments,UserSaves,UserMessages,UserMessagesReadall};
use share::common::AppState;
use share::common::Claims;


pub fn user_info(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let header = req.headers().get("Authorization").unwrap();
    let header_token = header.to_str().unwrap();
    let token = header_token[7..].to_string();
    let key = "secret";
    let validation = Validation {
        validate_exp: false,
        ..Default::default()
    };
    match decode::<Claims>(&token, key.as_ref(), &validation) {
        Ok(token_data) =>{
            req.state().db.send(UserInfo{user_id: token_data.claims.user_id.to_string()})
                .from_err()
                .and_then(|res| {
                    match res {
                        Ok(user_info) => {
                            Ok(HttpResponse::Ok().json(user_info))
                        },
                        Err(_) => Ok(HttpResponse::InternalServerError().into())
                    }
                }).responder()
        },
        Err(_) => Box::new(FutureResult(Ok(HttpResponse::InternalServerError().into()))),
    }
}

pub fn user_id((user_id, state): (Json<UserId>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(UserId{
        user_id: user_id.user_id,
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn user_delete(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let header = req.headers().get("Authorization").unwrap();
    let header_token = header.to_str().unwrap();
    let token = header_token[7..].to_string();
    let key = "secret";
    let validation = Validation {
        validate_exp: false,
        ..Default::default()
    };
    match decode::<Claims>(&token, key.as_ref(), &validation) {
        Ok(token_data) =>{
            req.state().db.send(UserDelete{user_id: token_data.claims.user_id.to_string()})
                .from_err()
                .and_then(|res| {
                    match res {
                        Ok(user_delete) => {
                            Ok(HttpResponse::Ok().json(user_delete))
                        },
                        Err(_) => Ok(HttpResponse::InternalServerError().into())
                    }
                }).responder()
        },
        Err(_) => Box::new(FutureResult(Ok(HttpResponse::InternalServerError().into()))),
    }
}

pub fn user_update((user_update, state): (Json<UserUpdate>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(UserUpdate{
        user_id: user_update.user_id,
        newname: user_update.newname.clone(),
        newmail: user_update.newmail.clone(),
        newpassword: user_update.newpassword.clone(),
        confirm_newpassword: user_update.confirm_newpassword.clone(),
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn user_update_img((user_update_img, state): (Json<UserUpdateImg>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(UserUpdateImg{
        user_id: user_update_img.user_id,
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn user_themes((user_themes, state): (Json<UserThemes>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(UserThemes{
        user_id: user_themes.user_id,
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn user_comments((user_comments, state): (Json<UserComments>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(UserComments{
        user_id: user_comments.user_id,
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn user_saves((user_saves, state): (Json<UserSaves>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(UserSaves{
        user_id: user_saves.user_id,
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn user_messages((user_messages, state): (Json<UserMessages>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(UserMessages{
        user_id: user_messages.user_id,
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn user_messages_readall((user_messages_readall, state): (Json<UserMessagesReadall>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(UserMessagesReadall{
        user_id: user_messages_readall.user_id,
        messages_unread_ids: user_messages_readall.messages_unread_ids.clone(),
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}
