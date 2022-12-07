use std::fs;

fn main() {
    let _example = String::from(
        "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
    );
    let _values = fs::read_to_string("input.txt").unwrap();
    //println!("part1: {}", part1(&_values));
    println!("part2: {}", part2(&_values));
    //println!("part1: {}", part1(&_example));
    //println!("part2: {}", part2(&_example));
}

fn part1(input: &str) -> u32 {
    let mut stacks: u32 = 0;
    let inst: Vec<&str> = input.split("\n\n").collect();
    let mut stacked: Vec<&str> = inst[0].split("\n").collect();

    // how many rows are there? not sure if I need this
    let rows: Vec<&str> = stacked.pop().unwrap().split(" ").collect();
    for r in rows {
        if r.len() > 0 {
            stacks += 1;
        }
    }

    let mut containers: [Vec<&str>; 9] = Default::default();
    for i in 0..stacked.len() {
        let cont = stacked.pop().unwrap();
        for element in 0..stacks {
            let row: usize = element as usize * 4 + 1;
            let mut temp: &str = &cont[row..(row + 1)];
            if temp != " " {
                containers[element as usize].push(temp);
            }
        }
    }

    let steps: Vec<&str> = inst[1].split("\n").collect();
    for step in steps {
        let temp: Vec<&str> = step.split(" ").collect();
        let quant = temp[1].parse::<usize>().unwrap();
        let from = temp[3].parse::<usize>().unwrap() - 1;
        let to = temp[5].parse::<usize>().unwrap() - 1;

        for q in 0..quant {
            let moving = containers[from].pop().unwrap();
            containers[to].push(moving);
        }
    }
    for cont in containers {
        println!("{}", cont.last().unwrap());
    }
    0
}

fn part2(input: &str) -> u32 {
    let mut stacks: u32 = 0;
    let inst: Vec<&str> = input.split("\n\n").collect();
    let mut stacked: Vec<&str> = inst[0].split("\n").collect();

    // how many rows are there? not sure if I need this
    let rows: Vec<&str> = stacked.pop().unwrap().split(" ").collect();
    for r in rows {
        if r.len() > 0 {
            stacks += 1;
        }
    }

    let mut containers: [Vec<&str>; 9] = Default::default();
    for i in 0..stacked.len() {
        let cont = stacked.pop().unwrap();
        for element in 0..stacks {
            let row: usize = element as usize * 4 + 1;
            let mut temp: &str = &cont[row..(row + 1)];
            if temp != " " {
                containers[element as usize].push(temp);
            }
        }
    }

    let steps: Vec<&str> = inst[1].split("\n").collect();
    for step in steps {
        let temp: Vec<&str> = step.split(" ").collect();
        let quant = temp[1].parse::<usize>().unwrap();
        let from = temp[3].parse::<usize>().unwrap() - 1;
        let to = temp[5].parse::<usize>().unwrap() - 1;
        let mut crane: Vec<&str> = Vec::new();

        for q in 0..quant {
            let moving = containers[from].pop().unwrap();
            crane.push(moving);
        }
        for q in 0..quant {
            let moving = crane.pop().unwrap();
            containers[to].push(moving);
        }
    }

    for cont in containers {
        print!("{}", cont.last().unwrap());
    }
    println!("");
    0
}
