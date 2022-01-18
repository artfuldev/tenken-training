use std::collections::VecDeque;
use std::fs::{OpenOptions};
use std::io::{Seek, SeekFrom, Read};
use actix::{ Actor, Context, Handler, Addr, ResponseActFuture, ActorFutureExt };
use fxhash::FxHashMap;

use crate::messages::{ LatestRequested, WriteRequested };
use crate::actors::Writer;

use super::Transactor;

pub struct Tenken {
    transactors_by_key: FxHashMap<String, Addr<Transactor>>,
    vacant_spots: VecDeque<Addr<Transactor>>,
    dummy_transactor: Addr<Transactor>
}

impl Tenken {
    pub fn new(capacity: u64, partition_size: u64, truncate: bool) -> Self {
        const MAX_HEADER_SIZE: usize = 110;
        let writer = Writer::new(capacity * partition_size, truncate).start();
        let mut vacant_spots = VecDeque::with_capacity(capacity.try_into().expect("capacity failed to fit in usize"));
        let mut transactors_by_key = FxHashMap::default();
        let mut file = 
            OpenOptions::new()
                .read(true)
                .open("db.dat")
                .expect("Unable to read database file");
        let mut header_buffer = [0; MAX_HEADER_SIZE];
        for index in 0..capacity {
            let mut transactor = Transactor::new(index, partition_size, writer.clone());
            file.seek(SeekFrom::Start(index * partition_size)).expect("seek partition failed");
            file.read_exact(&mut header_buffer).expect("read header failed");
            let key_size = usize::try_from(header_buffer[0]).expect("key bytes conversion to offset failed");
            if key_size == 0 {
                vacant_spots.push_back(transactor.start());
                continue;
            }
            let key = String::from_utf8(header_buffer[1..(key_size + 1)].to_vec()).expect("key read failed");
            let timestamp = u64::from_be_bytes(header_buffer[(key_size + 1)..(key_size + 9)].to_vec().try_into().expect(format!("failed to read timestamp for key {}", key).as_str()));
            transactor.restore(key.clone(), timestamp);
            transactors_by_key.insert(key, transactor.start());
        }
        let dummy_transactor = Transactor::new(0, capacity, writer.clone()).start();
        let vacancies = vacant_spots.len();
        println!("vacant_spots {}", vacancies);
        println!("capacity {}", capacity);
        Tenken {
            vacant_spots,
            transactors_by_key,
            dummy_transactor
        }
    }
}

impl Actor for Tenken {
    type Context = Context<Self>;
}

impl Handler<LatestRequested> for Tenken {
    type Result = ResponseActFuture<Self, Option<String>>;

    fn handle(&mut self, msg: LatestRequested, _ctx: &mut Self::Context) -> Self::Result {
        let LatestRequested(key) = msg;
        let transactor = self.transactors_by_key.get(&key).unwrap_or(&self.dummy_transactor);
        let send = transactor.send(LatestRequested(key));
        let future = actix::fut::wrap_future::<_, Self>(send);
        let update = future.map(|result, _, _| {
            match result {
                Ok(r) => r,
                Err(_) => None
            }
        });
        Box::pin(update)
    }
}

impl Handler<WriteRequested> for Tenken {
    type Result = ();

    fn handle(&mut self, msg: WriteRequested, _ctx: &mut Self::Context) -> Self::Result {
        match self.transactors_by_key.get(&msg.key) {
            None => {
                match self.vacant_spots.pop_front() {
                    None => (),
                    Some(addr) => {
                        self.transactors_by_key.insert(msg.key.clone(), addr.clone());
                        addr.do_send(msg);
                    }
                }
            },
            Some(addr) => {
                addr.do_send(msg);
            }
        }
    }
}
