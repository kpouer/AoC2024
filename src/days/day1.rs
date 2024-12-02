use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day1() -> Result<(), std::io::Error> {
    let file = File::open("data/d1.txt")?;
    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut tokenizer = line.split_whitespace();
            (
                tokenizer.next().unwrap().to_string(),
                tokenizer.next().unwrap().to_string(),
            )
        })
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .unzip();
    list1.sort();
    list2.sort();
    let distance = list1
        .iter()
        .zip(list2.iter())
        .map(|(x, y)| (x - y).abs())
        .reduce(|a, b| a + b)
        .unwrap();
    print!("Distance = {distance}");
    Ok(())
}
