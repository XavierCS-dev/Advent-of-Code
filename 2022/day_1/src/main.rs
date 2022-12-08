use std::{fs::File, io::{BufReader, self, BufRead}};

fn main() -> io::Result<()>{
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut sums: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let read = line.unwrap();
        if read.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += String::from(read).parse::<i32>().unwrap();
        }
    }

    let val = sums.iter().fold(0, |a, b| {if a > *b {a} else {*b}});
    println!("Part 1: {}", val);
    sums.remove(sums.iter().position(|value| *value == val).unwrap());
    let val2 = sums.iter().fold(0, |a, b| {if a > *b {a} else {*b}});
    sums.remove(sums.iter().position(|value| *value == val2).unwrap());
    let val3 = sums.iter().fold(0, |a, b| {if a > *b {a} else {*b}});
    println!("Part 2: {}", val + val2 + val3);
    Ok(())
}
