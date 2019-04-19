/// A universal return value for wrapping messages

use std::any::Any;


pub type ReturnValue = Box<Any + Send>;


#[cfg(test)]
mod tests {

    use super::*;

    struct TestVal {
        success: bool,
        msg: String,
    }

    #[test]
    fn test_return_value_should_create(){
        let r = TestVal{
            success: true,
            msg: String::from("ended"),
        };
        let b: ReturnValue = Box::new(r);
    }

    #[test]
    fn test_return_value_should_carry_a_message(){
        let r = TestVal{
            success: true,
            msg: String::from("ended"),
        };
        let b: ReturnValue = Box::new(r);
        let f: Box<TestVal> = b.downcast().unwrap();
        assert!(f.success);
        assert_eq!(f.msg, "ended");
    }
}