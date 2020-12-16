use std::fs;

fn traverse(slope: &Vec<&str>, right: i32, down: i32) -> i32 {
    let mut spot = 0;
    let mut trees = 0;
    let mut i = 0;

    while i < slope.len() {
        let line = slope[i];
        if line.len() == 0 {
            break;
        }
        if spot >= line.len() {
            spot = spot % line.len();
        }
        //println!("{} {}", spot, line);
        trees += if line.get(spot..(spot + 1)).unwrap_or(" ") == "#" {
            1
        } else {
            0
        };
        spot += right as usize;
        i += down as usize;
    }
    trees
}

fn main() {
    let slope = fs::read_to_string("day3_input.txt").expect("error reading file");
    let slope = slope.split("\n").collect::<Vec<&str>>();

    println!("part1: {}", traverse(&slope, 3, 1));

    let part2_product: i64 = traverse(&slope, 1, 1) as i64
        * traverse(&slope, 3, 1) as i64
        * traverse(&slope, 5, 1) as i64
        * traverse(&slope, 7, 1) as i64
        * traverse(&slope, 1, 2) as i64;

    println!("part2: {}", part2_product);
}
