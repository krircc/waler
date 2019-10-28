use actix_web::{fs::NamedFile, HttpRequest, Error, Result};
use share::common::AppState;

pub fn index(_req: &HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("public/index.html")?)
}

// pub fn error(_req: &HttpRequest<AppState>) -> Result<NamedFile> {
//     Ok(NamedFile::open("static/no.html")?)
// }