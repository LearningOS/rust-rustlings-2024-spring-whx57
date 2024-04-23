// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.



#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {  
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {  
        if value <= 0 {  
            // Check if value is zero or negative  
            if value == 0 {  
                Err(CreationError::Zero)  
            } else {  
                Err(CreationError::Negative)  
            }  
        } else {  
            // Value is positive and nonzero, so we can create a PositiveNonzeroInteger  
            Ok(PositiveNonzeroInteger(value as u64))  
        }  
    }  
}
#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
