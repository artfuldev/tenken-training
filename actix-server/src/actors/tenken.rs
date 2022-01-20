use std::collections::VecDeque;
use std::fs::OpenOptions;
use actix::{ Actor, Addr };
use fxhash::FxHashMap;
use stopwatch::Stopwatch;

use crate::actors::transactor::IndexedFileHandle;
use crate::messages::{ LatestRequested, WriteRequested };

use super::Transactor;

pub struct Tenken {
    transactors_by_key: FxHashMap<String, Addr<Transactor>>,
    vacant_spots: VecDeque<Addr<Transactor>>,
}

impl Tenken {
    pub fn new(capacity: u64) -> Self {
        let mut stopwatch = Stopwatch::start_new();
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
            let file = IndexedFileHandle::new(index, db_file.try_clone().expect("failed to clone db file handle"));
            let transactor = Transactor::new(file);
            match &transactor.state {
                None => {
                    vacant_spots.push_back(transactor.start())
                },
                Some((key, _)) => {
                    transactors_by_key.insert(key.clone(), transactor.start());
                }
            }
        }
        stopwatch.stop();
        let vacancies = vacant_spots.len();
        println!("vacant_spots {}", vacancies);
        println!("capacity {}", capacity);
        println!("db initialized in {}ms", stopwatch.elapsed_ms());
        Tenken {
            vacant_spots,
            transactors_by_key,
        }
    }

    pub async fn get(&self, key: String) -> Option<String> {
        match self.transactors_by_key.get(&key) {
            Some(t) =>
                match t.send(LatestRequested(key)).await {
                    Ok(x) => x,
                    Err(_) => None
                },
            None => None
        }
    }

    pub fn put(&mut self, key: String, value: String) -> () {
        match self.transactors_by_key.get(&key) {
            None => {
                match self.vacant_spots.pop_front() {
                    None => (),
                    Some(addr) => {
                        self.transactors_by_key.insert(key.clone(), addr.clone());
                        addr.do_send(WriteRequested { key, value });
                    }
                }
            },
            Some(addr) => {
                addr.do_send(WriteRequested { key, value });
            }
        }
    }
}
