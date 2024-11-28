//! This is a test project, you should ignore it.
//! 
//! It exists to check if docs.rs can build packages with no-std targets
//! specified.

#![no_std]

/// This is a function that adds two numbers.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
