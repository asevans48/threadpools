/// A threadpool where tasks perform no IO. This avoids some extra buildup.
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use num_cpus;
use crate::transport::{return_value, thunk, messages};
use crate::transport::return_value::ReturnValue;


/// Threadpool structure for accessing information
struct ThreadPool {
    num_threads: usize,
    threads: Vec<JoinHandle<()>>,
    workers: Vec<Sender<thunk::Thunk>>,
    stealers: Vec<Arc<Mutex<Receiver<thunk::Thunk>>>>,
    backend: Receiver<return_value::ReturnValue>,
}


/// Thread pool implementation
impl ThreadPool {

    fn submit(&self, thunk: thunk::Thunk, args: String) {

    }

    fn shutdown(&self) {

    }

    fn new(size: usize) {
        assert!(size > 0);
        let core_count = num_cpus::get();
        assert!(size <= core_count * 100);

        let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(size);
        let mut workers: Vec<Sender<thunk::Thunk>> = Vec::with_capacity(size);
        let mut stealers: Vec<Arc<Mutex<Receiver<thunk::Thunk>>>> = Vec::with_capacity(size);

        let (sx, rx): (Sender<ReturnValue>, Receiver<ReturnValue>) = mpsc::channel();

        for i in 0..size {
            /*
            let (wsx, wrx): (Sender<thunk::Thunk>, Receiver<thunk::Thunk>) = mpsc::channel();
            workers.push(wsx);
            let thread_receiver = Arc::new(Mutex::new(wrx));
            let thread_sx = sx.clone();
            let tid = i.clone();
            let handler = thread::spawn(move || {
                let mut run : bool = true;
                while run {
                    let data = thread_receiver.lock().unwrap().recv();
                    if data.is_ok(){
                        data.unwrap().call_box();
                    } else {
                        let m = messages::Terminated{
                            thread_id: tid,
                        };
                        let terminated: ReturnValue = Box::new(m);
                        thread_sx.send(terminated);
                        run = false;
                    }
                }
            });
            threads.push(handler);
            stealers.push(thread_receiver);
            */
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_should_create_pool(){

    }

    #[test]
    fn test_should_perform_work(){

    }

    #[test]
    fn test_should_perform_large_amounts_of_work(){

    }
}