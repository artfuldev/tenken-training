use actix::Message;

pub struct LatestRequested(pub String);

impl Message for LatestRequested {
    type Result = Option<String>;
}
