/// A threadpool where tasks perform no IO. This avoids some extra buildup.
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use crate::transport::{return_value, thunk};


/// Threadpool structure for accessing information
struct ThreadPool {
    num_threads: usize,
    threads: Vec<JoinHandle<()>>,
    workers: Vec<Sender<thunk::Thunk>>,
    backend: Receiver<return_value::ReturnValue>,
}


/// Thread pool implementation
impl ThreadPool {

    fn submit(&self, thunk: thunk::Thunk, args: String) {

    }

    fn shutdown() {

    }

    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut thread_vec: Vec<JoinHandle<()>> = Vec::with_capacity(size);
        let worker_queues: Vec<Sender<thunk::Thunk>> = Vec::with_capacity(size);
        let (backend_sender, backend_results): (Sender<return_value::ReturnValue>, Receiver<return_value::ReturnValue>) = mpsc::channel();

        for i in 0..size{
            let (work_sender, work_queue): (Sender<thunk::Thunk>, Receiver<thunk::Thunk>) = mpsc::channel();
            let safe_queue = Arc::new(Mutex::new(work_queue));
            let arc = safe_queue.clone();
            let thread_sender = backend_sender.clone();
            thread::spawn(move || {
                let q = arc.lock().unwrap();
                let work = (*q).recv();
                if work.is_ok(){

                }else{

                }
            });
        }

        ThreadPool {
            num_threads: size,
            threads: thread_vec,
            workers: worker_queues,
            backend: backend_results,
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_should_create_pool(){
        let mut pool = ThreadPool::new(10);
    }

    #[test]
    fn test_should_perform_work(){

    }

    #[test]
    fn test_should_perform_large_amounts_of_work(){

    }
}