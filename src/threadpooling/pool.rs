/// A threadpool where tasks perform no IO. This avoids some extra buildup.
use std::any::Any;
use std::sync::mpsc::{self, Sender, Receiver, RecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use num_cpus;
use crate::transport::{return_value, thunk, messages};
use crate::transport::return_value::ReturnValue;
use crate::transport::messages::{Terminate, Steal, Terminated};
use std::time::Duration;


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

    /// submit a function to the pool
    fn submit(&self, thunk: thunk::Thunk, args: String) {

    }

    fn terminate() -> i64 {
        0;
    }

    /// shutdown the pool
    fn shutdown(&self) {
        if !self.workers.is_empty() {
            for worker in self.workers{
                /* implemetn here */
            }

            let timeout = Duration::new(30, 0);
            for worker in self.workers{
                self.backend.recv_timeout(timeout);
            }
        }
    }

    /// create a new threadpool of the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let core_count = num_cpus::get();
        assert!(size <= core_count * 100);

        let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(size);
        let mut workers: Vec<Sender<thunk::Thunk>> = Vec::with_capacity(size);
        let mut stealers: Vec<Arc<Mutex<Receiver<thunk::Thunk>>>> = Vec::with_capacity(size);

        let (sx, rx): (Sender<ReturnValue>, Receiver<ReturnValue>) = mpsc::channel();

        for i in 0..size {
            let (wsx, wrx): (Sender<thunk::Thunk>, Receiver<thunk::Thunk>) = mpsc::channel();
            workers.push(wsx);
            let thread_receiver = Arc::new(Mutex::new(wrx));
            let thread_sx = sx.clone();
            let thread_rx = thread_receiver.clone();
            let tid = i.clone();
            let handler = thread::spawn(move || {
                let mut run : bool = true;
                let d = Duration::new(2, 0);
                let tidx = tid.to_owned();
                while run {
                    let data = thread_rx.lock().unwrap().recv_timeout(d);
                    if data.is_ok(){
                        let v = data.unwrap();
                        let rva: &Any = &v as &dyn Any;
                        if let Some(v) = rva.downcast_ref::<Terminate>(){
                            run = false;
                            let m= Terminated{
                                thread_id: tidx,
                            };
                            thread_sx.send(Box::new(m));
                        }else if let Some(v) = rva.downcast_ref::<thunk::Thunk>() {
                            v.call_box();
                        }
                    } else {
                        match data {
                            Err(RecvTimeoutError) => {
                                let m = Steal{
                                    thread_id:tidx,
                                };
                                thread_sx.send(Box::new(m));
                            }
                            Any =>{
                                run = false;
                                let m = Terminated{
                                    thread_id: tidx,
                                };
                                thread_sx.send(Box::new(m));
                            }
                        }
                    }
                }
            });
            threads.push(handler);
            stealers.push(thread_receiver);
        }

        ThreadPool {
            num_threads: size,
            threads: threads,
            workers: workers,
            stealers: stealers,
            backend: rx,
        }
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

    #[test]
    fn test_pool_should_shutdown(){

    }

    #[test]
    fn test_pool_should_do_work(){

    }
}