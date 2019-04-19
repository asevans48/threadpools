/// A universal return value for wrapping messages

use std::any::Any;


pub type ReturnValue = Box<Any + Send + 'static>;


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_return_value_should_create(){

    }

    #[test]
    fn test_return_value_should_carry_a_message(){

    }
}