// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO un-ignore the tests
    // then make them pass!

    #[test]
    #[ignore]
    fn test_non_digit_cc_number() {
        assert!(!luhn("foo"));
    }

    #[test]
    #[ignore]
    fn test_empty_cc_number() {
        assert!(!luhn(""));
        assert!(!luhn(" "));
        assert!(!luhn("  "));
        assert!(!luhn("    "));
    }

    #[test]
    #[ignore]
    fn test_single_digit_cc_number() {
        assert!(!luhn("0"));
    }

    #[test]
    #[ignore]
    fn test_two_digit_cc_number() {
        assert!(luhn(" 0 0 "));
    }

    #[test]
    #[ignore]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    #[ignore]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }
}

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn process_str(input: String) -> String {
    format!("{}", luhn(&input))
}

#[wasm_bindgen]
pub fn help_text() -> String {
    "Please enter a string to be processed.".to_string()
}
