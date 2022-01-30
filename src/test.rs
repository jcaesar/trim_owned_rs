use crate::r#impl::*;
use crate::*;

macro_rules! test {
    ($fn:ident) => {
        mod $fn {
            use super::*;
            fn test(input: &str) {
                println!("in: {input:?}");
                let correct = input.trim();
                println!("co: {correct:?} ({})", correct.len());
                let our = $fn(input.to_owned());
                let our = our.as_str();
                println!("ou: {our:?} ({})", our.len());
                assert_eq!(our, correct);
            }

            #[test]
            fn left() {
                test("    left")
            }
            #[test]
            fn right() {
                test("right    ")
            }
            #[test]
            fn both() {
                test("    both  ")
            }
            #[test]
            fn none() {
                test("none")
            }
        }
    };
}

test!(r#unsafe);
test!(middle);
test!(safe);
test!(naive);
test!(naive_opt);
test!(drain);

#[test]
fn no_realloc() {
    let input = "   both   ".to_string();
    let in_addr = input.as_ptr();
    let result = input.trim_owned();
    let out_addr = result.as_ptr();
    assert_eq!(in_addr, out_addr);
}
