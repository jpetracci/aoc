use std::fs;

fn part1(values: Vec<&str>) -> usize {
    let mut depth = 0;
    let mut horiz = 0;
    for m in values {
        let v = m.strip_suffix("\r").unwrap_or("");
        let mut v = v.split(" ");
        let v1 = v.next().unwrap();
        let v2 = v.next().unwrap_or("0");
        let v2 = v2.parse::<usize>().unwrap();
        match v1 {
            "down" => depth += v2,
            "up" => depth -= v2,
            "forward" => horiz += v2,
            _ => {}
        }
    }
    depth * horiz
}

fn part2(values: Vec<&str>) -> usize {
    let mut aim = 0;
    let mut depth = 0;
    let mut horiz = 0;
    for m in values {
        let v = m.strip_suffix("\r").unwrap_or("");
        let mut v = v.split(" ");
        let v1 = v.next().unwrap();
        let v2 = v.next().unwrap_or("0");
        let v2 = v2.parse::<usize>().unwrap();
        match v1 {
            "down" => aim += v2,
            "up" => aim -= v2,
            "forward" => {
                horiz += v2;
                depth += aim * v2;
            }
            _ => {}
        }
    }
    depth * horiz
}

fn main() {
    let values = fs::read_to_string("input.txt").expect("error reading file");
    let values = values.split("\n");
    let values = values.collect::<Vec<&str>>();

    println!("{}", part1(values.clone()));
    println!("{}", part2(values));
}
