use actix::Message;

pub struct LatestRequested;

impl Message for LatestRequested {
    type Result = Option<String>;
}
