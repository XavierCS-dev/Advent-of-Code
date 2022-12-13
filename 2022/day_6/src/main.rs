use std::fs;

fn part_1(data: String) -> Result<i32, String> {
    let mut total = 0;
    let mut characters: [char; 4] = ['0','0','0','0'];
    for character in data.chars() {
        unsafe {
            let first = characters.get_unchecked(0);
            let second = characters.get_unchecked(1);
            let third = characters.get_unchecked(2);
            let fourth = characters.get_unchecked(3);
            if first != second && first != third && first != fourth
            && second != third && second != fourth
            && third != fourth && first != &'0' {
                return Ok(total)
            }
        }
        characters[0] = characters[1];
        characters[1] = characters[2];
        characters[2] = characters[3];
        characters[3] = character;
        total += 1;
    }
    Err(String::from("Couldn't find 4 consecutive chars"))
}

fn part_2(data: String) -> Result<i32, String> {
    let mut characters: Vec<char> = Vec::new();
    let mut total = 0;
    for character in data.chars() {
        for i in 0..characters.len() {
            if *characters.get(i).unwrap() == character {
                characters.drain(0..(i+1));
                break;
            }
        }
        characters.push(character);
        total += 1;
        if characters.len() == 14 {
            return Ok(total)
        }
    }
    Err(String::from(format!{"Couldn't find 14 consecutive chars, searched {}", total}))
}

fn main() -> Result<(), std::io::Error> {
    // data is one line, so while file will be read
    let data = fs::read_to_string("data.txt")?;
    println!("Part 1: {}", part_1(data).unwrap());
    let data = fs::read_to_string("data.txt")?;
    println!("Part 2: {}", part_2(data).unwrap());
    Ok(())
}
