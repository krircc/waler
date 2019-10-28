use diesel::{self,RunQueryDsl,QueryDsl,ExpressionMethods};
use actix_web::{actix::Handler, error,Error};

use model::{user::User,theme::{Theme,Comment,Save}};
use model::info::{RusterInfo,RusterInfoMsgs};
use model::db::ConnDsl;


impl Handler<RusterInfo> for ConnDsl {
    type Result = Result<RusterInfoMsgs, Error>;

    fn handle(&mut self, ruster_info: RusterInfo, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        use utils::schema::themes;
        use utils::schema::comments;
        use utils::schema::saves;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let user_result =  users.load::<User>(conn).map_err(error::ErrorInternalServerError)?;
        let themes_result =  themes::table.load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        let blog_result =  themes::table.filter((themes::category_id).eq(2)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        let comments_result =  comments::table.load::<Comment>(conn).map_err(error::ErrorInternalServerError)?;
        let saves_result =  saves::table.load::<Save>(conn).map_err(error::ErrorInternalServerError)?;
        let mut ruster_info:Vec<u32> = vec![];
        ruster_info.push(user_result.len() as u32);
        ruster_info.push(themes_result.len() as u32);
        ruster_info.push(blog_result.len() as u32);
        ruster_info.push(comments_result.len() as u32);
        ruster_info.push(saves_result.len() as u32);
        Ok(RusterInfoMsgs{
                status: 200,
                message : "get  user_id info success.".to_string(),
                ruster_info: ruster_info,
        })
    }
}