use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // mpsc = multi-producer, single consumer
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Box up the function so that we can send it to a Worker that holds a receiver
        let job = Box::new(f);

        // We use `as_ref` here because we are not trying to take ownership of `sender` from
        // the ThreadPool value
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // drop sender first before anything dealing with the workers
        // note that we are calling `take` because we need ownership of the value
        // before we can call `drop` on it
        drop(self.sender.take());

        // Iterate through all the workers and joining them
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        // loop {...} is an infinite loop, and each thread will just be hanging around
        // and waiting for something to come to its receiver
        let thread = thread::spawn(move || loop {
            // receiver.lock() works because Arc implements the Deref trait
            // A Mutex needs to call lock() first before it can use the value inside
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break; // this ends the infinite loop and terminates the thread closure
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
