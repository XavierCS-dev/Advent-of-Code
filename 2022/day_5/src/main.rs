use std::{fs::File, io::{BufReader, BufRead}};


// part 1 and 2 are identical excluding the change of one for loop.
// The use of unwrap is okay, as it should never fail with the correct input file, 
// and the program should panic if it fails, because it cannot operate on poorly formatted data
// Since the parts are nearly identical, the failure only needs to be indicated in part 1,
// and if someone modifies this code to only run part 2, they will likely see this comment.
fn part_1(reader: BufReader<File>, mut stacks: [Vec<&str>;9]) -> Result<String, std::io::Error>{
        let err_str = "Data is in the wrong format";
        for line in reader.lines() {
            let line_data = line?;
            let data: Vec<&str> = line_data.split(" ").collect();
            let (amt, source, destination) = (String::from(*data.get(1).expect(err_str)), 
                String::from(*data.get(3).expect(err_str)), 
                String::from(*data.get(5).expect(err_str)));
            let mut elf_crates: Vec<&str> = Vec::new();
            for _ in 0..(amt.parse::<i32>().expect(err_str)) {
                elf_crates.push(stacks[source.parse::<usize>().expect(err_str) - 1].pop().unwrap())
            }
            for item in &elf_crates {
                stacks[destination.parse::<usize>().expect(err_str) - 1].push(*item);
            }
        }
        let mut top = String::new();
        for elf_stack in &mut stacks {
            top.push_str(elf_stack.pop().unwrap());
        }
        Ok(top)
}

fn part_2(reader: BufReader<File>, mut stacks: [Vec<&str>;9]) -> Result<String, std::io::Error>{
        for line in reader.lines() {
            let line_data = line?;
            let data: Vec<&str> = line_data.split(" ").collect();
            let (amt, source, destination) = (String::from(*data.get(1).unwrap()), 
                String::from(*data.get(3).unwrap()), 
                String::from(*data.get(5).unwrap()));
            let mut elf_crates: Vec<&str> = Vec::new();
            for _ in 0..(amt.parse::<i32>().unwrap()) {
                elf_crates.push(stacks[source.parse::<usize>().unwrap() - 1].pop().unwrap())
            }
            for _ in 0..elf_crates.len() {
                stacks[destination.parse::<usize>().unwrap() - 1].push(elf_crates.pop().unwrap());
            }
        }
        let mut top = String::new();
        for elf_stack in &mut stacks {
            top.push_str(elf_stack.pop().unwrap());
        }
        Ok(top)
}

fn get_stack() -> [Vec<&'static str>;9] {
    // You will need to create 9 vectors, containing the stacks,
    // manually or otherwise
    // data.txt should only contain the instructions
    let stack_1 = vec!["C","Z","N","B","M","W","Q", "V"];
    let stack_2 = vec!["H","Z","R","W","C","B"];
    let stack_3 = vec!["F","Q","R","J"];
    let stack_4 = vec!["Z","S","W","H","F","N","M","T"];
    let stack_5 = vec!["G","F","W","L","N","Q","P"];
    let stack_6 = vec!["L","P","W"];
    let stack_7 = vec!["V","B","D","R","G","C","Q","J"];
    let stack_8 = vec!["Z","Q","N","B","W"];
    let stack_9 = vec!["H","L","F","C","G","T","J"];
    [stack_1, stack_2, stack_3, stack_4, stack_5, stack_6, stack_7, stack_8, stack_9]
}

fn main() {
    let file = File::open("data.txt").expect("Could not open file, is it missing?");
    let reader = BufReader::new(file);
    println!("part 1: {}", part_1(reader, get_stack()).unwrap());
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    println!("part 2: {}", part_2(reader, get_stack()).unwrap());
}
