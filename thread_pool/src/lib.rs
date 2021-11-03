use std::thread;

mod worker;

use worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new Thread Pool.
    /// The size is the number of the threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The ´new´ function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Must have at least one thread");
        }

        let mut workers = Vec::with_capacity(size);

        for id in ..size {
            workers.push(Worker::new(id));
        }

        Self { workers }
    }
}
