use std::collections::VecDeque;
use std::fs::{OpenOptions};
use actix::{ Actor, Context, Handler, Addr, ResponseActFuture, ActorFutureExt };
use fxhash::FxHashMap;

use crate::actors::transactor::{IndexedFileHandle, FileHandle};
use crate::messages::{ LatestRequested, WriteRequested };

use super::Transactor;

pub struct Tenken {
    transactors_by_key: FxHashMap<String, Addr<Transactor>>,
    vacant_spots: VecDeque<Addr<Transactor>>,
    dummy_transactor: Addr<Transactor>
}

impl Tenken {
    pub fn new(capacity: u64) -> Self {
        let db_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.dat")
            .expect("Unable to open database file");
        db_file
            .set_len(capacity * 2048)
            .expect("Unable to set length of database file");
        let mut vacant_spots = VecDeque::with_capacity(capacity.try_into().expect("capacity failed to fit in usize"));
        let mut transactors_by_key = FxHashMap::default();
        for index in 0..capacity {
            let mut file = IndexedFileHandle::new(index);
            println!("created file handle {}", index);
            match file.read_preface().unwrap_or(None) {
                None => vacant_spots.push_back(Transactor::new(file).start()),
                Some(preface) => {
                    let mut transactor = Transactor::new(file);
                    transactor.restore(preface.key.clone(), preface.timestamp);
                    transactors_by_key.insert(preface.key.clone(), transactor.start());
                }
            }
        }
        let dummy_transactor = Transactor::new(IndexedFileHandle::new(0)).start();
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
