use std::fs;

fn parse_data1(data: &String) {
    let data = data.split("\n").collect::<Vec<&str>>();
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut field_count = 0;
    let mut valid_passports = 0;

    for line in data {
        if line.len() == 0 {
            if field_count == 7 {
                valid_passports += 1;
            }
            field_count = 0;
        }
        for field in &fields {
            if line.contains(field) {
                field_count += 1;
            }
        }
    }

    println!("part1: {}", valid_passports);
}

fn parse_data2(data: String) {
    let data = data.split("\n").collect::<Vec<&str>>();
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut field_count = 0;
    let mut valid_passports = 0;

    for line in data {
        // check for end of passport entry
        if line.len() == 0 {
            if field_count == 7 {
                valid_passports += 1;
            }
            field_count = 0;
        }
        for field in &fields {
            if line.contains(field) {
                let sep_fs = line.split(" ").collect::<Vec<&str>>();
                for f in sep_fs {
                    if f.contains(field) {
                        //println!("{:?}", f);
                        let f = f.split(":").collect::<Vec<&str>>();

                        match field as &str {
                            "byr" => {
                                if f[1].len() == 4 {
                                    let val = f[1].parse::<i32>().unwrap_or(0);
                                    if val >= 1920 && val <= 2002 {
                                        field_count += 1;
                                    }
                                }
                            }
                            "iyr" => {
                                if f[1].len() == 4 {
                                    let val = f[1].parse::<i32>().unwrap_or(0);
                                    if val >= 2010 && val <= 2020 {
                                        field_count += 1;
                                    }
                                }
                            }
                            "eyr" => {
                                if f[1].len() == 4 {
                                    let val = f[1].parse::<i32>().unwrap_or(0);
                                    if val >= 2020 && val <= 2030 {
                                        field_count += 1;
                                    }
                                }
                            }
                            "hgt" => {
                                if f[1].contains("cm") {
                                    let val = f[1].split("c").collect::<Vec<&str>>();
                                    let val = val[0].parse::<i32>().unwrap_or(0);
                                    if val >= 150 && val <= 193 {
                                        field_count += 1;
                                    }
                                } else if f[1].contains("in") {
                                    let val = f[1].split("i").collect::<Vec<&str>>();
                                    let val = val[0].parse::<i32>().unwrap_or(0);
                                    if val >= 59 && val <= 76 {
                                        field_count += 1;
                                    }
                                }
                            }
                            "hcl" => {
                                if f[1].len() == 7 {
                                    let val = f[1].split("#").collect::<Vec<&str>>();
                                    //let val = i64::from_str_radix(f[1], 16);
                                    let val = hex::decode(val[1]);
                                    match val {
                                        Ok(_) => field_count += 1,
                                        Err(_) => (),
                                    };
                                }
                            }
                            "ecl" => match f[1] {
                                "amb" => field_count += 1,
                                "blu" => field_count += 1,
                                "brn" => field_count += 1,
                                "gry" => field_count += 1,
                                "grn" => field_count += 1,
                                "hzl" => field_count += 1,
                                "oth" => field_count += 1,
                                _ => (),
                            },
                            "pid" => {
                                if f[1].len() == 9 {
                                    field_count += 1
                                }
                            }
                            _ => (),
                        };
                    }
                }
            }
        }
    }

    println!("part2: {}", valid_passports);
}

fn main() {
    let passports = fs::read_to_string("day4_input.txt").expect("error reading file");
    //let passports = fs::read_to_string("test_input.txt").expect("error reading file");
    parse_data1(&passports);
    parse_data2(passports);
}
