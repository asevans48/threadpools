/// A thunk capable of sharing routines between threads
/// @author aevans
use std::any::Any;
use serde::ser::{Serialize, Serializer, SerializeStruct};

pub trait FnBox {
    fn call_box(self: Box<Self>);
}


impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}


pub type Thunk = Box<dyn FnBox + Send>;


pub trait FnBoxGeneric<T> {
    fn call_box(self: Box<Self>) -> Option<T>;
}


impl <F: FnOnce() -> i64> FnBoxGeneric<i64> for F {
    fn call_box(self: Box<F>) -> Option<i64> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> bool> FnBoxGeneric<bool> for F {
    fn call_box(self: Box<F>) -> Option<bool> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> char> FnBoxGeneric<char> for F {
    fn call_box(self: Box<F>) -> Option<char> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> i32> FnBoxGeneric<i32> for F {
    fn call_box(self: Box<F>) -> Option<i32> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> i16> FnBoxGeneric<i16> for F {
    fn call_box(self: Box<F>) -> Option<i16> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> isize> FnBoxGeneric<isize> for F {
    fn call_box(self: Box<F>) -> Option<isize> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> u8> FnBoxGeneric<u8> for F {
    fn call_box(self: Box<F>) -> Option<u8> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> u16> FnBoxGeneric<u16> for F {
    fn call_box(self: Box<F>) -> Option<u16> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> u32> FnBoxGeneric<u32> for F {
    fn call_box(self: Box<F>) -> Option<u32> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> u64> FnBoxGeneric<u64> for F {
    fn call_box(self: Box<F>) -> Option<u64> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> usize> FnBoxGeneric<usize> for F {
    fn call_box(self: Box<F>) -> Option<usize> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> f32> FnBoxGeneric<f32> for F {
    fn call_box(self: Box<F>) -> Option<f32> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> f64> FnBoxGeneric<f64> for F {
    fn call_box(self: Box<F>) -> Option<f64> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> String> FnBoxGeneric<String> for F {
    fn call_box(self: Box<F>) -> Option<String> {
        Some((*self)())
    }
}


impl <F: FnOnce() -> Vec<u8>> FnBoxGeneric<Vec<u8>> for F {
    fn call_box(self: Box<F>) -> Option<Vec<u8>> {
        Some((*self)())
    }
}


pub type ReturnableThunk<T> = Box<dyn FnBoxGeneric<T> + Send>;


#[cfg(test)]
mod tests {

    use super::*;
    use serde_derive::{Serialize, Deserialize};
    use std::thread::{self, Thread};

    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        id: i8,
        key: i16,
        name: String,
    }

    fn execute(b: bool) {
        assert!(true);
    }

    #[test]
    fn test_returnable_thunk_serde(){
        let t = TestStruct{
            id: 0,
            key: 112,
            name: "TestStruct".to_string(),
        };
        let bytes = bincode::serialize(&t).unwrap();
        let f: ReturnableThunk<Vec<u8>> = Box::new(move|| bytes);
        let d = f.call_box();
        assert!(d != None);
        assert!(d.is_some());
        let mut bcode = d.unwrap();
        let obj:TestStruct = bincode::deserialize(&bcode).unwrap();
        assert!(obj.id == 0);
        assert!(obj.key == 112);
        assert!(obj.name.eq("TestStruct"));
    }

    #[test]
    fn test_returnable_thunk_string(){
        let f: ReturnableThunk<String> = Box::new(|| String::from("Hello"));
        let d = f.call_box();
        assert!(d != None);
        assert!(d.is_some());
        assert!(d.unwrap().eq("Hello"));
    }

    #[test]
    fn test_returnable_thunk_i64(){
        let f: ReturnableThunk<i64> = Box::new(|| 1 + 1);
        let d = f.call_box();
        assert!(d != None);
        assert!(d.is_some());
        assert!(d.unwrap() == 2);
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