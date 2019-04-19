/// Messages for transporting return values


trait TypeInfo {
    fn type_of(&self) -> &'static str;
}


///Message type for general return values
struct Message {
    thread_id: i32,
    msg: String,
}


impl TypeInfo for Message {
    fn type_of(&self) -> &'static str {
        "Message"
    }
}



///Steal instruction, should cause work stealing
struct Steal{
    thread_id: i32,
}


impl TypeInfo for Steal {
    fn type_of(&self) -> &'static str {
        "stealer"
    }
}


///Termination signal
struct Terminated {
    thread_id: i32,
}


impl TypeInfo for Terminated {
    fn type_of(&self) -> &'static str {
        "terminated"
    }
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