use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandler<()>>,
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

        let threads = Vec::with_capacity(size);

        Self { threads }
    }
}
