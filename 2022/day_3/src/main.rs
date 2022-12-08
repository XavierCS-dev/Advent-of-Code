use std::{collections::HashMap, io::{BufReader, BufRead, Read}, fs::File, time::SystemTime, error::Error};

fn get_values_hashmap() -> HashMap<char, i32> {
    let mut values: HashMap<char, i32> = HashMap::new();
    let mut iter_val = 1;
    for character in "abcdefghijklmnopqrstuvwxyz".chars() {
        values.insert(character, iter_val);
        iter_val += 1;    
    }
    for character in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        values.insert(character, iter_val);
        iter_val += 1;
    }
    values
}

fn part_1(values: &HashMap<char, i32>, reader: BufReader<File>) {
    let mut total = 0;
    for line in reader.lines() {
        match line {
            Ok(string) => {
                let (container1, container2) = string.split_at(string.len() / 2);
                for letter in container1.chars() {
                    if container2.contains(letter) {
                        total += values.get(&letter).unwrap();
                        break;
                    }
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
    println!("total part 1: {}", total);
}

fn part_2(values: &HashMap<char, i32>, reader: BufReader<File>) -> Result<(), Box<dyn Error>>{
    let mut total = 0;
    let mut all_lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        all_lines.push(line?);
    }
    while !all_lines.is_empty() {
        let line1 = all_lines.pop().unwrap();
        let line2 = all_lines.pop().unwrap();
        let line3 = all_lines.pop().unwrap();
        for character in line1.chars() {
            if line2.contains(character) && line3.contains(character) {
                total += values.get(&character).unwrap();
                break;
            }
        }
    }
    println!("Part 2 total: {}", total);
    Ok(())
}
    
fn main() {
    let values = get_values_hashmap();
    match File::open("data.txt") {
        Ok(file) => {
            let reader = BufReader::new(file);
            part_1(&values, reader);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
    match File::open("data.txt") {
        Ok(file) => {
            let reader = BufReader::new(file);
            part_2(&values, reader);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
