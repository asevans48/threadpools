/// General messages

///Message type for general return values
pub struct Message {
    pub thread_id: usize,
    pub msg: String,
}


///Steal instruction, should cause work stealing
pub struct Steal {
    pub thread_id: usize,
}

///Termination signal
pub struct Terminated {
    pub thread_id: usize,
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::transport::return_value;
    use std::any::Any;
    use std::error::Error;

    #[test]
    fn test_should_deconstruct_message_to_testable_type(){
        let steal = Steal{
            thread_id: 1,
        };
        let rval: return_value::ReturnValue = Box::new(steal);
        let sv: Box<Steal> = rval.downcast().unwrap();
        assert!(sv.thread_id == 1);
    }

    #[test]
    fn test_should_be_able_to_check_message_type() {
        let steal = Steal {
            thread_id: 1,
        };
        let rval: return_value::ReturnValue = Box::new(steal);
        let rva: &Any = (rval.as_ref()) as &dyn Any;
        if let Some(rval) = rva.downcast_ref::<Steal>() {
            assert!(true);
        }else {
            assert!(false);
        }
    }
}