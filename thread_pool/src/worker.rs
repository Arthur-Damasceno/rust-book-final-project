use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;
type Receiver = Arc<Mutex<mpsc::Receiver<Message>>>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Receiver) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker #{} got a job.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker #{} was told to terminate.", id);
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
