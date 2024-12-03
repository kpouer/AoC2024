use crate::days::d3::operation::Operation;
use std::fs;
use std::io::Read;

pub(crate) fn day3() -> i32 {
    let line = fs::read_to_string("data/d3.txt").unwrap();
    let mut result = 0;
    let mut text: &str = &line;
    loop {
        match text.find("mul(") {
            None => break,
            Some(pos) => {
                text = &text[pos + 4..];
                match text.find(')') {
                    None => break,
                    Some(rparens) => {
                        let mul = &text[..rparens];
                        println!("mul = {mul}");
                        if let Ok(operation) = Operation::try_from(mul) {
                            result += operation.compute();
                            text = &text[rparens..];
                        }
                    }
                }
            }
        }
    }
    result
}

mod test {
    use super::*;

    #[test]
    fn test_day3() {
        assert_eq!(163931492, day3());
    }
    // #[test]
    // fn test_day3p3() {
    //     assert_eq!(343, day2_part2().unwrap());
    // }
}
