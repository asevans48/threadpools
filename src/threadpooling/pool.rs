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
use std::cell::Cell;
use std::collections::hash_map::HashMap;
use std::sync::mpsc::{self, Sender, Receiver, SendError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, current};
use num_cpus;
use crate::transport::thunk;
use crate::transport::return_value::ReturnValue;
use crate::transport::messages::{self, Signal, Steal, Terminate, Terminated};
use std::time::Duration;
use core::borrow::{BorrowMut, Borrow};


/// A worker struct
struct Worker{
    sender: Sender<thunk::Thunk>,
    thread: Option<JoinHandle<()>>,
}

/// Threadpool structure for accessing information
struct ThreadPool {
    size: usize,
    pool_channel: mpsc::Sender<thunk::Thunk>,
    pool_signal: mpsc::Sender<messages::Signal>,
}


/// Thread pool implementation
impl ThreadPool {

    /// shuts down the pool gracefully
    fn shutdown(self){
        let t: Terminate = Terminate{};
        let rm: Signal = Box::new(t);
        self.pool_signal.send(rm);
    }

    /// Submit a task onto the pool. Returns the submission result.
    fn submit(self, thunk: thunk::Thunk) -> Result<(), SendError<thunk::Thunk>> {
        self.pool_channel.send(thunk)
    }


    /// Create a worker which performs tasks
    fn create_worker(thread_id: usize, monitor_sender: Sender<Signal>, backend: Sender<ReturnValue>) -> (JoinHandle<()>, Sender<thunk::Thunk>, Arc<Mutex<Receiver<thunk::Thunk>>>, Sender<messages::Signal>) {
        let (sender, receiver): (Sender<thunk::Thunk>, Receiver<thunk::Thunk>) = mpsc::channel();
        let (sgn_sender, sgn_receiver): (Sender<messages::Signal>, Receiver<messages::Signal>) = mpsc::channel();
        let arc_receiver = Arc::new(Mutex::new(receiver));
        let thread_receiver = arc_receiver.clone();
        let thread_signaler = monitor_sender.clone();
        let mut handle =thread::spawn(move ||{
            let tidx = thread_id.clone();
            let mut run: bool = true;
            while run {
                let sgn_data = sgn_receiver.try_recv();
                if sgn_data.is_ok() {
                    let sgn_rval = sgn_data.unwrap();
                    let sgn_data_any = sgn_rval.as_ref() as &dyn Any;
                    if let Some(sgn_data) = sgn_data_any.downcast_ref::<Terminate>(){
                        run = false;
                    }
                }
                if run {
                    let duration = Duration::new(15, 0);
                    let data = thread_receiver.lock().unwrap().recv_timeout(duration);
                    if data.is_ok() {
                        let rdata = data.unwrap();
                        rdata.call_box();
                    }else if data.is_err(){
                        let m = Steal{
                          thread_id: tidx,
                        };
                        let sval: Signal = Box::new(m);
                        thread_signaler.send(sval);
                    }
                }
            }
        });
        (handle, sender, arc_receiver, sgn_sender)
    }

    /// Creates a worker, encapsulates the run methods
    fn create_workers(size: usize, backend: Sender<ReturnValue>) -> (HashMap<usize, Worker>, Vec<Arc<Mutex<Receiver<thunk::Thunk>>>>, Vec<Sender<messages::Signal>>, Receiver<Signal>, Sender<Signal>){
        let mut worker_map: HashMap<usize, Worker> = HashMap::<usize, Worker>::new();
        let mut stealers: Vec<Arc<Mutex<Receiver<thunk::Thunk>>>> = Vec::with_capacity(size.clone());
        let mut signalers: Vec<Sender<messages::Signal>> = Vec::with_capacity(size);
        let (monitor_signaler, monitor_receiver): (Sender<Signal>, Receiver<Signal>) = mpsc::channel();
        for i in 0..size {
            let tidx: usize = i.clone();
            let (handle, sender, receiver, signaler): (JoinHandle<()>, Sender<thunk::Thunk>, Arc<Mutex<Receiver<thunk::Thunk>>>, Sender<messages::Signal>) = ThreadPool::create_worker(tidx, monitor_signaler.clone(),backend.clone());
            let mut cell = Some(handle);
            let mut witem = Worker{
                sender: sender.clone(),
                thread: cell,
            };
            worker_map.insert(tidx, witem);
            signalers.push(signaler);
            stealers.push(receiver);

        }
        (worker_map, stealers, signalers, monitor_receiver, monitor_signaler.clone())
    }

    /// Run the master thread which maintains workers and monitors threads.
    fn run_master(size: usize, master_receiver: Receiver<thunk::Thunk>, master_signaler: Receiver<messages::Signal>) -> (JoinHandle<()>, Receiver<ReturnValue>){
        let (backend_sender, backend_receiver): (Sender<ReturnValue>, Receiver<ReturnValue>) = mpsc::channel();
        let thread_backend = backend_sender.clone();

        let nthreads: usize = size.clone();
        let master = thread::spawn(move ||{
            let (mut workers,
                mut stealers,
                mut signalers,
                monitor_receiver,
                monitor_signaler) = ThreadPool::create_workers(nthreads, thread_backend.clone());
            let mut current_thread = 0;
            let mut run: bool = true;
            while run {
                // check for user signals
                let sgn_data = master_signaler.try_recv();
                if sgn_data.is_ok(){
                    let sgn_rval = sgn_data.unwrap();
                    let sgn_data_any = sgn_rval.as_ref() as &dyn Any;
                    if let Some(sgn_data) = sgn_data_any.downcast_ref::<Terminate>(){
                        run = false;
                    }
                }

                // check for thread originated signals
                let thread_data = monitor_receiver.try_recv();
                if thread_data.is_ok() {
                    let udata = thread_data.unwrap();
                    let val_any = udata.as_ref() as &dyn Any;
                    match val_any.downcast_ref::<Box<Terminated>>(){
                        Some(m) => {
                            let tidx = m.thread_id.clone();
                            workers.remove(&tidx);
                            let (handle, sender, receiver, signaler): (JoinHandle<()>, Sender<thunk::Thunk>, Arc<Mutex<Receiver<thunk::Thunk>>>, Sender<messages::Signal>) = ThreadPool::create_worker(tidx, monitor_signaler.clone(),backend_sender.clone());
                            let mut cell = Some(handle);
                            let mut witem = Worker{
                                sender: sender.clone(),
                                thread: cell,
                            };
                            workers.insert(tidx, witem);
                        }
                        None => {

                        }
                    }
                }


                // handle incoming data
                let d = Duration::new(5, 0);
                let data = master_receiver.recv_timeout(d);
                if data.is_ok(){
                    if current_thread >= size{
                        current_thread = 0;
                    }
                    let mut witem: Option<&Worker> = workers.get(&current_thread);
                    if witem.is_none() {
                        assert!(current_thread < size);
                        while witem.is_none() {
                            current_thread += 1;
                            witem = workers.get(&current_thread);
                        }
                    }else{
                        current_thread += 1;
                    }
                    let fn_box = data.unwrap();
                    witem.unwrap().sender.send(fn_box);
                }
            }
            // wait for threads to terminate
            let keys = workers.keys();
            for i in 0 .. keys.len(){
                let mut w = workers.get_mut(&i);
                if w.is_some() {
                    let mut t = w.unwrap();
                    t.thread.take().unwrap().join();
                }
            }
        });
        (master, backend_receiver)
    }

    /// Run the threadpool. Returns the thread handle for the pool.
    fn run(size: usize) -> (JoinHandle<()>, ThreadPool, Receiver<ReturnValue>) {
        let (master_sender, master_receiver): (Sender<thunk::Thunk>, Receiver<thunk::Thunk>) = mpsc::channel();
        let (master_signal, master_signal_receiver): (Sender<messages::Signal>, Receiver<messages::Signal>) = mpsc::channel();
        let pool_channel = master_sender.clone();
        let (master, backend) = ThreadPool::run_master(size.clone(), master_receiver, master_signal_receiver);
        let pool = ThreadPool {
            size: size,
            pool_channel: master_sender.clone(),
            pool_signal: master_signal.clone(),
        };
        (master, pool, backend)
    }

    /// Create a new threadpool of the specified size
    fn new(size: usize) -> (JoinHandle<()>, ThreadPool, Receiver<ReturnValue>) {
        assert!(size > 0);
        let core_count = num_cpus::get();
        assert!(size <= core_count * 100);
        ThreadPool::run(size)
    }
}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_should_create_pool(){
        let p = ThreadPool::new(3);
    }
}