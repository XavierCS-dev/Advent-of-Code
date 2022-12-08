use std::{io::BufReader, fs::File};
use std::io::BufRead;


fn get_ranges (line: String) -> (i32, i32, i32, i32) {
    let ranges = line.split(",").collect::<Vec<_>>();
    let range1 = ranges.get(0).unwrap().split("-").collect::<Vec<_>>();
    let range2 = ranges.get(1).unwrap().split("-").collect::<Vec<_>>();
    let r1 = range1.get(0).unwrap().parse::<i32>().unwrap();
    let r2 = range1.get(1).unwrap().parse::<i32>().unwrap();
    let r3 = range2.get(0).unwrap().parse::<i32>().unwrap();
    let r4 = range2.get(1).unwrap().parse::<i32>().unwrap();
    (r1, r2, r3, r4)
}

fn part_1(reader: BufReader<File>) -> Result<i32, std::io::Error> {
    let mut total = 0;
    for line in reader.lines() {
        let new_line = String::from(line?);
        let (r1, r2, r3, r4) = get_ranges(new_line);
        if (r1 >= r3 && r2 <= r4) || (r1 <= r3 && r2 >= r4) {
            total += 1
        }
    }

    Ok(total)
}

fn part_2(reader: BufReader<File>) -> Result<i32, std::io::Error> {
    let mut total = 0;
    for line in reader.lines() {
        let new_line = String::from(line?);
        let (r1, r2, r3, r4) = get_ranges(new_line);
        if (r1 >= r3 && r1 <= r4) || (r2 >= r3 && r2 <= r4) || (r3 >= r1 && r3 <= r2) || (r4 >= r1 && r4 <= r2) {
            total += 1;
        }
    }

    Ok(total)
}

fn main() {
    let file = File::open("data.txt").expect("Failed to open data.txt. Is it missing?");
    let reader = BufReader::new(file);
    println!("part 1: {}", part_1(reader).unwrap());
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    println!("part 2: {}", part_2(reader).unwrap());
}