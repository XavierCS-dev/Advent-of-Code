use std::{io::{self, BufReader, BufRead}, fs::File, error};

struct RockPaperScissors {
    rock_win: i32,
    paper_win: i32,
    scissors_win: i32,
    rock_lose: i32,
    paper_lose: i32,
    scissors_lose: i32,
    rock_draw: i32,
    paper_draw: i32,
    scissors_draw: i32,
}

fn main() {
    let game = RockPaperScissors::new(7, 8, 9, 1, 2, 3, 4, 5, 6);
    println!("{}", game.calculate_points("data.txt").unwrap());
    
}

impl RockPaperScissors {
    fn new(rock_win: i32, paper_win: i32, scissors_win: i32, rock_lose: i32, paper_lose: i32, scissors_lose: i32, 
        rock_draw: i32, paper_draw: i32, scissors_draw: i32) -> RockPaperScissors {

        RockPaperScissors {
            rock_win,
            paper_win,
            scissors_win,
            rock_lose,
            paper_lose,
            scissors_lose,
            rock_draw,
            paper_draw,
            scissors_draw,
        }
    }

    fn calculate_points(&self, path: &str) -> Result<i32, io::Error> {
        let mut points = 0;
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                
                for line in reader.lines() {
                    let new_l = line.unwrap();
                    let vals: Vec<&str> = new_l.split(" ").collect();
                    let opponenet = *vals.get(0).unwrap();
                    let player = *vals.get(1).unwrap();
                    match (opponenet, player) {
                        ("A", "Z") => {
                            points += self.paper_win
                        }
                        ("A", "X") => {
                            points += self.scissors_lose
                        }
                        ("A", "Y") => {
                            points += self.rock_draw
                        }
                        ("B", "X") => {
                            points += self.rock_lose
                        }
                        ("B", "Y") => {
                            points += self.paper_draw
                        }
                        ("B", "Z") => {
                            points += self.scissors_win
                        }
                        ("C", "Y") => {
                            points += self.scissors_draw
                        }
                        ("C", "Z") => {
                            points += self.rock_win
                        }
                        ("C", "X") => {
                            points += self.paper_lose
                        }
                        (_, _) => {
                            println!("points: {}", points);
                            println!("Player: {}", player);
                            println!("Opponenet: {}", opponenet);
                            panic!("Found something unusual")
                        }
                    }
                }            
            }
            Err(error) => return Err(error)
        } 
        Ok(points)   
    }
}
