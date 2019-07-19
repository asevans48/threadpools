use num_cpus;
use core::borrow::{Borrow, BorrowMut};
use std::any::Any;
use std::cell::Cell;
use std::char;
use std::collections::hash_map::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, SendError};
use std::thread;
use std::thread::{current, JoinHandle};
use std::time::Duration;

use rand::prelude::*;

use crate::transport::messages::{self, Signal, Steal, Terminate, Terminated};
use crate::transport::return_value::ReturnValue;
use crate::transport::thunk;
use crate::transport::thunk::{FnBoxGeneric, ReturnableThunk};
use crate::transport::return_value::ReturnMessage;
use crate::threadpooling::pool::ThreadPool;

pub mod threadpooling;
pub mod transport;
pub mod asyncpooling;

fn main() {
    let (jh, mut tp, receiver) = ThreadPool::<i64>::new(3);
    for i in 0..10000 {
        let thunk: ReturnableThunk<i64> = Box::new(|| -> i64 {
            let mut s: String = "Hello ".to_string();
            s.push_str("World");
            let mut rng = rand::thread_rng();
            let endF: f64 = rng.gen();
            let endVal = ((endF * 1000.0) as i64);
            if (endF < 0.5) {
                for i in 0..endVal {
                    s.push_str(i.to_string().as_str());
                }
            }
            1 + 1
        });
        assert!(tp.submit(thunk).is_ok());
    }
    let run = true;
    while run{

    }
    //tp.shutdown();
}
