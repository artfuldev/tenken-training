use std::time::Duration;
use actix::{Actor, Addr, Context, ActorStreamExt, ContextFutureSpawner};
use crate::writer::{Writer, Log};
use actix::utils::IntervalFunc;

pub(crate) struct Tick {
    writer: Addr<Writer>
}

impl Tick {
    pub fn new(writer: Addr<Writer>) -> Self {
        Tick { writer }
    }

    fn tick(&mut self, _ctx: &mut Context<Self>) -> () {
        self.writer.do_send(Log::Debug("tick".to_string()));
    }
}

impl Actor for Tick {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        IntervalFunc::new(Duration::from_millis(100), Self::tick)
           .finish()
           .spawn(ctx);
    }
}
