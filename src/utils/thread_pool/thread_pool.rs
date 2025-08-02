use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// `size = 0`,  panic.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

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
    thread: Option<thread::JoinHandle<()>>, // Option can transfer ownership.
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic(expected = "assertion failed: size > 0")]
    fn test_thread_pool_zero_size_panics() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_job_execution() {
        let pool = ThreadPool::new(2);
        let result = Arc::new(Mutex::new(Vec::new()));
        let result_clone = Arc::clone(&result);

        pool.execute(move || {
            result_clone.lock().unwrap().push(1);
        });

        // Give the thread pool time to execute the job
        thread::sleep(Duration::from_millis(100));

        let final_result = result.lock().unwrap();
        assert_eq!(*final_result, vec![1]);
    }

    #[test]
    fn test_multiple_jobs_execution() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            });
        }

        // Wait for all jobs to complete
        thread::sleep(Duration::from_millis(200));

        let final_count = *counter.lock().unwrap();
        assert_eq!(final_count, 10);
    }

    #[test]
    fn test_concurrent_job_execution() {
        let pool = ThreadPool::new(3);
        let shared_data = Arc::new(Mutex::new(Vec::new()));

        for i in 0..5 {
            let data_clone = Arc::clone(&shared_data);
            pool.execute(move || {
                thread::sleep(Duration::from_millis(50)); // Simulate work
                data_clone.lock().unwrap().push(i);
            });
        }

        // Wait for all jobs to complete
        thread::sleep(Duration::from_millis(300));

        let final_data = shared_data.lock().unwrap();
        assert_eq!(final_data.len(), 5);
        
        // Check that all values 0-4 are present (order may vary due to concurrency)
        for i in 0..5 {
            assert!(final_data.contains(&i));
        }
    }

    #[test]
    fn test_thread_pool_drop() {
        let pool = ThreadPool::new(2);
        let executed = Arc::new(Mutex::new(false));
        let executed_clone = Arc::clone(&executed);

        pool.execute(move || {
            *executed_clone.lock().unwrap() = true;
        });

        // Give time for execution
        thread::sleep(Duration::from_millis(100));

        // Pool will be dropped here, testing the Drop implementation
        drop(pool);

        assert!(*executed.lock().unwrap());
    }

    #[test]
    fn test_worker_thread_execution() {
        let pool = ThreadPool::new(1);
        let start_time = std::time::Instant::now();
        let duration_result = Arc::new(Mutex::new(None));
        let duration_clone = Arc::clone(&duration_result);

        pool.execute(move || {
            let elapsed = start_time.elapsed();
            *duration_clone.lock().unwrap() = Some(elapsed);
        });

        thread::sleep(Duration::from_millis(100));

        let duration = duration_result.lock().unwrap();
        assert!(duration.is_some());
        assert!(duration.unwrap() < Duration::from_millis(200));
    }
}
