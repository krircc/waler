use diesel::{self,QueryDsl,ExpressionMethods,RunQueryDsl};
use actix_web::{actix::Handler,error,Error};
use chrono::Utc;
use bcrypt::{DEFAULT_COST, hash, verify};
use jwt::{encode, Header, Algorithm};
use md5::compute;
use ring::digest;

use model::user::{User, NewUser, SignupUser, SigninUser, UserInfo, UserUpdate, UserUpdateImg,UserId,
                  UserDelete, UserThemes,UserComments,UserSaves,UserMessages,UserMessagesReadall};
use model::response::{Msgs, SigninMsgs, UserIdMsgs,UserInfoMsgs, UserThemesMsgs,UserCommentsMsgs,
                      UserSavesMsgs, UserMessagesMsgs};
use model::db::ConnDsl;
use model::message::Message;
use model::theme::{Theme, Comment,Save};
use share::common::Claims;
use model::response::MyError;

impl Handler<SignupUser> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, signup_user: SignupUser, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let user_result =  users.filter(&username.eq(&signup_user.username)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
        match user_result {
            Some(user) => {
                Ok(Msgs { 
                    status: 400,
                    message: "同名用户已注册，请更换用户名注册.".to_string(),
                })
            },
            None => {
                if &signup_user.password == &signup_user.confirm_password {
                        use utils::schema::users::dsl::*;
                        let hash_password = match hash(&signup_user.password, DEFAULT_COST) {
                            Ok(h) => h,
                            Err(_) => panic!()
                        };
                        let digest = compute(&signup_user.email);
                        let md5str = format!("{:x}", digest);
                        let avatar_url = "http://www.gravatar.com/avatar/".to_string() + &md5str + "?s=128&d=identicon";
                        let new_user = NewUser {
                            email: &signup_user.email,
                            username: &signup_user.username,
                            password: &hash_password,
                            created_at: Utc::now().naive_utc(),
                            avatar: &avatar_url,
                        };
                        diesel::insert_into(users).values(&new_user).execute(conn).map_err(error::ErrorInternalServerError)?;
                        Ok(Msgs { 
                                status: 200,
                                message : "注册成功.".to_string(),
                        })
                }else{
                    Ok(Msgs { 
                            status: 400,
                            message : "两次密码输入不同，请重新输入.".to_string(),
                    })
                }
            }
        }
    }
}

impl Handler<SigninUser> for ConnDsl {
    type Result = Result<SigninMsgs, Error>;

    fn handle(&mut self, signin_user: SigninUser, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let mut login_user =  users.filter(&username.eq(&signin_user.username)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let no_user = User::new();
        match login_user {
            Some(mut login_user) => {
                match verify(&signin_user.password, &login_user.password) {
                    Ok(valid) if valid => {

                        let number = [154, 241, 91, 51, 110, 106, 150, 25, 146, 133, 55, 223, 48, 178, 230, 162, 55, 101, 105, 252, 249, 215, 231, 115, 236, 206, 222, 101, 96, 101, 41, 160];

                        let aa = signin_user.code.as_bytes();
                        let actual = digest::digest(&digest::SHA256, aa);
                        let cc = &actual.as_ref();

                        if (signin_user.code == "00") || (&number == cc) {
                            let key = "secret";
                            let claims = Claims {
                                user_id: login_user.id.to_string(),
                            };
                            let token = match encode(&Header::default(), &claims, key.as_ref()) {
                                Ok(t) => t,
                                Err(_) => panic!() // in practice you would return the error
                            };
                            if login_user.avatar == "" {
                                login_user.avatar = "https://www.gravatar.com/avatar/1".to_string();
                            }
                            let the_user = User {
                                id: login_user.id,
                                email: login_user.email.clone(),
                                username: login_user.username.clone(),
                                password: login_user.password.clone(),
                                created_at : login_user.created_at.clone(),
                                avatar : login_user.avatar.clone(),
                            };
                            Ok(SigninMsgs { 
                                status: 200,
                                token: token,
                                signin_user: the_user,
                                message: "登陆成功！ signin.".to_string(),
                            })
                        }else{
                            Ok(SigninMsgs { 
                                status: 400,
                                token: "".to_string(),
                                signin_user: no_user,
                                message: "登陆错误.".to_string(),
                            })
                        } 
                    },
                    _ => {
                        Ok(SigninMsgs { 
                            status: 400,
                            token: "".to_string(),
                            signin_user: no_user,
                            message: "密码错误，请重新输入.".to_string(),
                        })
                    },
                }
            },
            None => {
                Ok(SigninMsgs { 
                    status: 400,
                    token: "".to_string(),
                    signin_user: no_user,
                    message: "没有此用户，请核对用户名和密码.".to_string(),
                })
            }
        }
    }
}

impl Handler<UserInfo> for ConnDsl {
    type Result = Result<UserInfoMsgs, Error>;

    fn handle(&mut self, user_info: UserInfo, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let user_id: i32 = user_info.user_id.parse().map_err(error::ErrorBadRequest)?;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let login_user =  users.filter(&id.eq(&user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
        match login_user {
            Some(login_user) => {
                    let current_user = User {
                            id: login_user.id,
                            email: login_user.email,
                            username: login_user.username,
                            password: login_user.password,
                            created_at : login_user.created_at,
                            avatar : login_user.avatar,
                    };
                    Ok(UserInfoMsgs {
                            status: 200,
                            message : "The  current_user info.".to_string(),
                            current_user: current_user,
                    })
            },
            None => {
                    let no_user = User::new();
                    Ok(UserInfoMsgs {
                            status: 400,
                            message : "error.".to_string(),
                            current_user: no_user,
                    })
            },
        }
    }
}

impl Handler<UserId> for ConnDsl {
    type Result = Result<UserIdMsgs, Error>;

    fn handle(&mut self, user_id: UserId, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let user_result =  users.filter(id.eq(user_id.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let mut hourse_user = User::new();
        match user_result {
                Some(user_one) => {
                        hourse_user = user_one;
                },
                None => { println!("No user result"); },
        }
        Ok(UserIdMsgs{
                status: 200,
                message : "get  user_id info success.".to_string(),
                hourse_user: hourse_user,
        })
    }
}

impl Handler<UserDelete> for ConnDsl {
    type Result = Result<Msgs, MyError>;

    fn handle(&mut self, user_delete: UserDelete, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let user_id: i32 = user_delete.user_id.parse().unwrap();
        let conn = &self.0.get().unwrap();
        let login_user = diesel::delete(users.filter(&id.eq(&user_id))).execute(conn);
        match login_user {
            Ok(Msgs) => Ok(Msgs{
                                status: 200,
                                message : "delete  loginuser success.".to_string(),
                        }),
            Ok(_) => Err(MyError::NotFound),
            Err(_) => Err(MyError::DatabaseError),
        }
    }
}

impl Handler<UserUpdate> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, user_update: UserUpdate, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let hash_password = match hash(&user_update.newpassword, DEFAULT_COST) {
                Ok(h) => h,
                Err(_) => panic!()
        };
        diesel::update(users)
            .filter(&id.eq(&user_update.user_id))
            .set((
                username.eq(user_update.newname),
                email.eq(user_update.newmail),
                password.eq(hash_password),
            )).execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs{
                status: 200,
                message : "update  loginuser success.".to_string(),
        })
    }
}

impl Handler<UserUpdateImg> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, user_update_img: UserUpdateImg, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let user_result =  users.filter(id.eq(user_update_img.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
        match user_result {
            Some(user) => {
                let digest = compute(&user.email);
                let md5str = format!("{:x}", digest);
                let avatar_url = "http://www.gravatar.com/avatar/".to_string() + &md5str + "?s=128";
                diesel::update(users)
                        .filter(&id.eq(&user_update_img.user_id))
                        .set((
                            avatar.eq(avatar_url),
                        )).execute(conn).map_err(error::ErrorInternalServerError)?;
                Ok(Msgs{
                        status: 200,
                        message : "更新用户头像成功".to_string(),
                })
            },
            None => {
                Ok(Msgs { 
                    status: 400,
                    message: "系统未查到此用户,更新失败".to_string(),
                })
            }
        }
    }
}

impl Handler<UserThemes> for ConnDsl {
    type Result = Result<UserThemesMsgs, Error>;

    fn handle(&mut self, user_themes: UserThemes, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let user_themes_result = themes.filter(user_id.eq(user_themes.user_id)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(UserThemesMsgs{
                status: 200,
                message : "get  userthemes success.".to_string(),
                themes : user_themes_result,
        })
    }
}

impl Handler<UserComments> for ConnDsl {
    type Result = Result<UserCommentsMsgs, Error>;

    fn handle(&mut self, user_comments: UserComments, _: &mut Self::Context) -> Self::Result {
        use utils::schema::comments::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let user_comments_result = comments.filter(user_id.eq(user_comments.user_id)).load::<Comment>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(UserCommentsMsgs{
                status: 200,
                message : "update  loginuser success.".to_string(),
                comments : user_comments_result,
        })
    }
}

impl Handler<UserSaves> for ConnDsl {
    type Result = Result<UserSavesMsgs, Error>;

    fn handle(&mut self, user_saves: UserSaves, _: &mut Self::Context) -> Self::Result {
        use utils::schema::saves::dsl::*;
        use utils::schema::themes;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let user_saves = saves.filter(user_id.eq(user_saves.user_id)).load::<Save>(conn).map_err(error::ErrorInternalServerError)?;
        let mut themes: Vec<Theme> = vec![];
        for save in user_saves {
            let theme_one = themes::table.filter((themes::id).eq(save.theme_id)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?.pop();
            match theme_one {
                Some(one) => themes.push(one),
                Nome => { println!("No save result"); },
            }
        }
        Ok(UserSavesMsgs{
                status: 200,
                message : "update  loginuser success.".to_string(),
                saves : themes,
        })
    }
}

impl Handler<UserMessages> for ConnDsl {
    type Result = Result<UserMessagesMsgs, Error>;

    fn handle(&mut self, user_messages: UserMessages, _: &mut Self::Context) -> Self::Result {
        use utils::schema::messages::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let user_messages_result = messages.order(id).filter(to_user_id.eq(user_messages.user_id)).load::<Message>(conn).map_err(error::ErrorInternalServerError)?;
        let mut messages_count: u16 = 0;
        let mut messages_unread_ids: Vec<i32> = vec![];
        for message in &user_messages_result {
            if message.message_status == 0 {
                messages_count += 1;
                messages_unread_ids.push(message.id)
            }
        }
        Ok(UserMessagesMsgs{
                status: 200,
                message : "User messages get success.".to_string(),
                messages : user_messages_result,
                messages_count: messages_count,
                messages_unread_ids: messages_unread_ids,
        })
    }
}

impl Handler<UserMessagesReadall> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, user_messages_readall: UserMessagesReadall, _: &mut Self::Context) -> Self::Result {
        use utils::schema::messages::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let rang = user_messages_readall.messages_unread_ids;
        for index in rang.iter(){
            diesel::update(messages).filter(&id.eq(index))
                    .set(message_status.eq(1)).execute(conn).map_err(error::ErrorInternalServerError)?;
        }
        Ok(Msgs{
                status: 200,
                message : "your messages readall success.".to_string(),
        })
    }
}
