use crate::days::d2::report::Report;
use crate::util::tokenize_line;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day2() -> Result<usize, std::io::Error> {
    let file = File::open("data/d2.txt")?;
    let valid_reports = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| tokenize_line(&line))
        .map(|vec| {
            vec.iter()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(Report::from)
        .filter(|report| report.is_valid())
        .count();
    print!("Valid report = {valid_reports}");
    Ok(valid_reports)
}

pub(crate) fn day2_part2() -> Result<usize, std::io::Error> {
    let file = File::open("data/d2.txt")?;
    let valid_reports = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| tokenize_line(&line))
        .map(|vec| {
            vec.iter()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(Report::from)
        .filter(|report| report.is_valid2())
        .count();
    print!("Valid report = {valid_reports}");
    Ok(valid_reports)
}

mod test {
    use super::*;

    #[test]
    fn test_day2() {
        assert_eq!(279, day2().unwrap());
    }
    #[test]
    fn test_day2p2() {
        assert_eq!(343, day2_part2().unwrap());
    }
}
