use actix::Message;

pub struct WriteAtRequested {
    pub offset: u64,
    pub data: Vec<u8>
}

impl Message for WriteAtRequested {
    type Result = ();
}
