use actix::Message;

pub struct WriteRequested(pub String, pub String);

impl Message for WriteRequested {
    type Result = ();
}
