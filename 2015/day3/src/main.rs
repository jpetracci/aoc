use std::fs;

// Location, x, y, visited
#[derive(PartialEq, PartialOrd)]
struct Location(i32, i32, i32);

fn main() {
    let data = fs::read_to_string("input.txt").expect("did not open file");
    // let data = String::from("^v^v^v^v^v");
    let commands: Vec<char> = data.chars().collect();

    let mut houses: Vec<Location> = Vec::new();

    // first house has been visited already
    houses.push(Location(0, 0, 1));

    // set current position
    let mut x = 0;
    let mut y = 0;

    for command in &commands {

        // execute comand
        match command {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => println!("unkown command"),
        }
        // println!("{} {} {}", command, x, y);

        // check if house is in the list already
        if houses
            .iter()
            .position(|current_house| current_house == &Location(x, y, 1))
            .is_none()
            == true
        {
            // add house to the list
            houses.push(Location(x, y, 1));
        }
    }
    println!("num house = {}", houses.len());
}

// fn execute_command(c: char) {}
