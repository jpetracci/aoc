use std::fs;

fn main() {
    let _example = String::from(
        "
A Y
B X
C Z
",
    );
    let values = fs::read_to_string("input.txt").expect("file error");
    println!("part1: {}", part1(values.clone()));
    println!("part2: {}", part2(values));
}

fn part1(input: String) -> u32 {
    let mut result: u32 = 0;
    let game = input.split("\n").collect::<Vec<&str>>();
    for g in game {
        let round = g.split(" ").collect::<Vec<&str>>();
        if round.len() == 2 {
            match round[0] {
                "A" => {
                    if round[1] == "X" {
                        // rock .. rock .. draw
                        result += 1;
                        result += 3;
                    }
                    if round[1] == "Y" {
                        // rock .. paper .. win
                        result += 2;
                        result += 6;
                    }
                    if round[1] == "Z" {
                        // rock .. scissors .. loss
                        result += 3;
                        result += 0;
                    }
                }
                "B" => {
                    if round[1] == "X" {
                        // paper .. rock .. loss
                        result += 1;
                        result += 0;
                    }
                    if round[1] == "Y" {
                        // paper .. paper .. draw
                        result += 2;
                        result += 3;
                    }
                    if round[1] == "Z" {
                        // paper .. scissors .. win
                        result += 3;
                        result += 6;
                    }
                }
                "C" => {
                    if round[1] == "X" {
                        // scissors .. rock .. win
                        result += 1;
                        result += 6;
                    }
                    if round[1] == "Y" {
                        // scissors .. paper .. loss
                        result += 2;
                        result += 0;
                    }
                    if round[1] == "Z" {
                        // scissors .. scissors .. draw
                        result += 3;
                        result += 3;
                    }
                }

                _ => (),
            }
        }
    }
    result
}

fn part2(input: String) -> u32 {
    let mut result: u32 = 0;
    let game = input.split("\n").collect::<Vec<&str>>();
    for g in game {
        let round = g.split(" ").collect::<Vec<&str>>();
        if round.len() == 2 {
            match round[0] {
                "A" => {
                    if round[1] == "Y" {
                        // rock .. rock .. draw
                        result += 1;
                        result += 3;
                    }
                    if round[1] == "Z" {
                        // rock .. paper .. win
                        result += 2;
                        result += 6;
                    }
                    if round[1] == "X" {
                        // rock .. scissors .. loss
                        result += 3;
                        result += 0;
                    }
                }
                "B" => {
                    if round[1] == "X" {
                        // paper .. rock .. loss
                        result += 1;
                        result += 0;
                    }
                    if round[1] == "Y" {
                        // paper .. paper .. draw
                        result += 2;
                        result += 3;
                    }
                    if round[1] == "Z" {
                        // paper .. scissors .. win
                        result += 3;
                        result += 6;
                    }
                }
                "C" => {
                    if round[1] == "Z" {
                        // scissors .. rock .. win
                        result += 1;
                        result += 6;
                    }
                    if round[1] == "X" {
                        // scissors .. paper .. loss
                        result += 2;
                        result += 0;
                    }
                    if round[1] == "Y" {
                        // scissors .. scissors .. draw
                        result += 3;
                        result += 3;
                    }
                }

                _ => (),
            }
        }
    }
    result
}
