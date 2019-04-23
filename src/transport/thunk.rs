/// A thunk capable of sharing routines between threads
/// @author aevans


pub trait FnBox {
    fn call_box(self: Box<Self>);
}


impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}


pub type Thunk = Box<dyn FnBox + Send>;


#[cfg(test)]
mod tests {

    use super::*;
    use std::thread::Thread;

    fn execute(b: bool) {
        assert!(true);
    }

    #[test]
    fn test_thunk(){
        let f: Thunk = Box::new(|| assert!(true));
        f.call_box();
    }

    #[test]
    fn test_exec_func_in_thunk() {
        let b : bool = true;
        let f: Thunk = Box::new(move|| execute(b));
        f.call_box();
    }

    #[test]
    fn test_exec_func_in_thunk_threads() {
        for i in 0..10{
            let f: Thunk = Box::new(|| execute(true));
            thread::spawn(move || {
                f.call_box();
            });
        }
    }
}