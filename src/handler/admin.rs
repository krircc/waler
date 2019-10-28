use actix_web::{actix::Handler, error,Error};
use diesel::{self,RunQueryDsl,QueryDsl,ExpressionMethods};
use model::db::ConnDsl;
use model::user::{User,AdminUsers};
use model::theme::{Theme,AdminThemes};
use model::response::{AdminUsersMsgs,AdminThemesMsgs};

impl Handler<AdminUsers> for ConnDsl {
    type Result = Result<AdminUsersMsgs, Error>;

    fn handle(&mut self, admin_users: AdminUsers, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let user_result =  users.order(id).load::<User>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(AdminUsersMsgs{
                status: 200,
                message : "get  user_id info success.".to_string(),
                admin_users: user_result,
        })
    }
}

impl Handler<AdminThemes> for ConnDsl {
    type Result = Result<AdminThemesMsgs, Error>;

    fn handle(&mut self, admin_themes: AdminThemes, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let themes_result =  themes.order(id).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(AdminThemesMsgs{
                status: 200,
                message : "get  user_id info success.".to_string(),
                admin_themes: themes_result,
        })
    }
}