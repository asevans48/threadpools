/// A thunk capable of sharing routines between threads
/// @author aevans


trait FnBox {
    fn call_box(self: Box<Self>);
}


impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}


type Thunk = Box<dyn FnBox + Send + 'static>;


#[cfg(test)]
mod tests {

    use super::*;

    fn execute(b: bool) {
        assert!(b);
    }

    #[test]
    fn test_thunk(){
        let f: Thunk = Box::new(|| assert!(true));
        f.call_box();
    }

    #[test]
    fn test_exec_func_in_thunk() {
        let f: Thunk = Box::new(|| execute(true));
        f.call_box();
    }

    #[test]
    fn test_exec_func_in_thunk_threads() {

    }
}