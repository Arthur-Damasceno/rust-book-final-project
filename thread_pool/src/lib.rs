use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

mod worker;

use worker::{Job, Worker};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn execute<T>(&self, f: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
