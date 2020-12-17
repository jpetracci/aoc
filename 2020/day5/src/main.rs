use std::fs;

fn calc_highest(seats: String) {}

fn main() {
    //let seats = fs::read_to_string("input.txt").expect("error reading file");
    let seats = fs::read_to_string("test.txt").expect("error reading file");
    let highest = calc_highest(seats);
}
