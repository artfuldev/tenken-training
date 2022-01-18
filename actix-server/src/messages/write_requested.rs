use actix::Message;

pub struct WriteRequested {
    pub key: String,
    pub value: String
}

impl Message for WriteRequested {
    type Result = ();
}
