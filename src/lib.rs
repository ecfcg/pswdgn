mod generator;
pub use generator::cli;
pub use generator::Generator;

use std::ffi::CString;
use std::os::raw::c_char;

pub extern "C" fn generate(length: usize, usable_code: usize, is_easy_code: usize) -> *mut c_char {
    match generator::validate_length(length as i128) {
        Ok(_) => (),
        Err(_) => panic!("unsupported lunght: {}", length),
    }

    let validated_usable_code =
        usable_code & generator::character::category::Category::all_flagged_code();
    let is_easy = is_easy_code & (1 as usize) == 1;

    let generated = Generator::new(length as u8, validated_usable_code, is_easy).generate();
    let cs = CString::new(generated).expect("CString::new failed");
    cs.into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_len_8() {
        let result1;
        let result2;
        let result3;
        unsafe {
            result1 = String::from_raw_parts(generate(8, 15, 1) as *mut u8, 8, 8);
            result2 = String::from_raw_parts(generate(8, 15, 1) as *mut u8, 8, 8);
            result3 = String::from_raw_parts(generate(8, 15, 1) as *mut u8, 8, 8);
        }

        assert_eq!(result1.len(), 8);
        assert_eq!(result2.len(), 8);
        assert_eq!(result3.len(), 8);

        assert_ne!(result1, result2);
        assert_ne!(result1, result3);
    }
    #[test]
    fn it_works_len_15() {
        let result1;
        let result2;
        let result3;
        unsafe {
            result1 = String::from_raw_parts(generate(15, 3, 0) as *mut u8, 15, 15);
            result2 = String::from_raw_parts(generate(15, 3, 0) as *mut u8, 15, 15);
            result3 = String::from_raw_parts(generate(15, 3, 0) as *mut u8, 15, 15);
        }

        assert_eq!(result1.len(), 15);
        assert_eq!(result2.len(), 15);
        assert_eq!(result3.len(), 15);

        assert_ne!(result1, result2);
        assert_ne!(result1, result3);
    }

    #[test]
    #[should_panic(expected = "unsupported lunght: 7")]
    fn it_works_panic() {
        generate(7, 3, 0);
    }
}
