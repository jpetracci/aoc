use std::fs;

fn part1(values: Vec<&str>) -> usize {
    let mut counts = vec![0; 12];

    // count 1s
    for row in values.clone() {
        for (i, c) in row.chars().enumerate() {
            match c {
                '1' => counts[i] += 1,
                _ => (),
            }
        }
    }
    // simplify len
    let total = values.len();
    // get new value in binary vec
    let gamma: Vec<usize> = counts.iter().map(|f| (f > &(total / 2)) as usize).collect();
    let eps: Vec<usize> = counts.iter().map(|f| (f < &(total / 2)) as usize).collect();
    // convert the str
    let gamma: &str = &gamma.into_iter().map(|v| v.to_string()).collect::<String>();
    let eps: &str = &eps.into_iter().map(|v| v.to_string()).collect::<String>();
    // convert to integer
    let gamma = usize::from_str_radix(gamma, 2).unwrap();
    let eps = usize::from_str_radix(eps, 2).unwrap();
    // end value
    gamma * eps
}

fn filter(values: Vec<&str>, bit: usize, ox: u8) -> Vec<&str> {
    let mut counts = 0;
    let mut end: Vec<&str> = Vec::new();
    let mut keep = 0;

    if values.len() < 2 {
        return values;
    }

    // count 1s
    for row in values.clone() {
        match row.chars().nth(bit).unwrap_or(' ') {
            '1' => counts += 1,
            _ => (),
        }
    }

    if ox == 1 {
        // keep 1's if more common or equal
        if counts as f32 >= values.len() as f32 / 2.0 {
            keep = 1;
        }
    } else {
        // keep 1's if less common otherwise 0's
        if (counts as f32) < values.len() as f32 / 2.0 {
            keep = 1;
        }
    }

    // keep values with 1's or 0's
    for row in values.clone() {
        if keep == 1 {
            match row.chars().nth(bit).unwrap_or(' ') {
                '1' => end.push(row),
                _ => (),
            }
        } else {
            match row.chars().nth(bit).unwrap_or(' ') {
                '0' => end.push(row),
                _ => (),
            }
        }
    }

    end
}

fn part2(values: Vec<&str>) -> usize {
    let mut v = values.clone();
    for n in 0..12 {
        v = filter(v.clone(), n, 1);
    }
    // convert to integer
    let ox = usize::from_str_radix(v[0].strip_suffix("\r").unwrap(), 2).unwrap();
    println!("{:?}", ox);

    let mut v = values.clone();
    for n in 0..12 {
        v = filter(v.clone(), n, 0);
    }
    // convert to integer
    let co2 = usize::from_str_radix(v[0].strip_suffix("\r").unwrap(), 2).unwrap();
    println!("{:?}", co2);

    ox * co2
}

fn main() {
    let values = fs::read_to_string("input.txt").expect("error reading file");
    //let values =
    //    "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
    let values = values.split("\n");
    let values = values.collect::<Vec<&str>>();

    println!("part 1 = {}", part1(values.clone()));
    println!("part 2 = {}", part2(values));
}
