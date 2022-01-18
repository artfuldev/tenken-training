use actix::{ Actor, Context, Handler };

use crate::messages::{ LatestRequested, WriteRequested };

pub struct Tenken;

impl Actor for Tenken {
    type Context = Context<Self>;
}

impl Handler<LatestRequested> for Tenken {
    type Result = Option<String>;

    fn handle(&mut self, msg: LatestRequested, ctx: &mut Self::Context) -> Self::Result {
        None
    }
}

impl Handler<WriteRequested> for Tenken {
    type Result = ();

    fn handle(&mut self, msg: WriteRequested, ctx: &mut Self::Context) -> Self::Result {
        
    }
}
