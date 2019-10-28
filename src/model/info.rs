use actix_web::{Error,actix::Message};

#[derive(Deserialize,Serialize, Debug)]
pub struct RusterInfo;

impl Message for RusterInfo {
    type Result = Result<RusterInfoMsgs, Error>;
}

#[derive(Deserialize,Serialize, Debug)]
pub struct RusterInfoMsgs {
    pub status: i32,
    pub message : String,
    pub ruster_info : Vec<u32>,
}