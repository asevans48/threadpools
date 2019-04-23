/// A threadpool that provides different functionality compared to Rayon and Tokio.
/// This threadpool is meant to continue to run and process individual tasks.
/// As with Tokio, each worker has a thread. However, no I/O is expected. There
/// is no overhead for I/O monitoring as there is with await and async. This pool
/// should form the basis of a large scale task processor. While each thread
/// remains active, adequate amount of work need to be passed to avoid expenses.
/// There is no based on work-load at runtime. This pool does not break down large
/// tasks for parallel execution. In fact, this pool operates on thunks instead of
///parallel iterators.
use std::any::Any;
use std::sync::mpsc::{self, Sender, Receiver, RecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use num_cpus;
use crate::transport::thunk;
use crate::transport::return_value::ReturnValue;
use crate::transport::messages;
use std::time::Duration;


/// Threadpool structure for accessing information
struct ThreadPool {
    health_receiver: Receiver<messages::Signal>,
    signal_queues: Vec<mpsc::Receiver<messages::Signal>>,
    size: usize,
    stealers: Vec<Arc<Mutex<mpsc::Receiver<thunk::Thunk>>>>,
    workers: Vec<JoinHandle<()>>,
    worker_queues: Vec<mpsc::Sender<thunk::Thunk>>,
}


/// Thread pool implementation
impl ThreadPool {

    fn create_pool(size: usize) -> ThreadPool {

        ThreadPool{
            health_receiver: None,
            signal_queues: None,
            size: 0,
            stealers: None,
            workers: None,
            worker_queues: None,
        }
    }

    /// create a new threadpool of the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let core_count = num_cpus::get();
        assert!(size <= core_count * 100);
        ThreadPool::create_pool(size)
    }
}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_should_create_pool(){
        let p = ThreadPool::new(3);
        assert!(p.num_threads == 3);
    }
}