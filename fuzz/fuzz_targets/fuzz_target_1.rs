#![no_main]
use libfuzzer_sys::fuzz_target;
use trim::TrimOwned;

fuzz_target!(|data: &str| { assert_eq!(data.trim(), data.to_owned().trim_owned().as_str()) });
