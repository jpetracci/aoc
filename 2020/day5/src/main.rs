use std::cmp;
use std::fs;

fn calc_highest(seats: String) -> i32 {
    let seats = seats.lines();
    let mut highest = 0;
    let mut seat_list = Vec::new();
    for seat in seats {
        let spots = seat.chars();
        let mut row_end = 127;
        let mut row_start = 0;
        let mut column_end = 7;
        let mut column_start = 0;

        for spot in spots {
            // println!(
            //     "spot {}, plane {} {}, row {} {}",
            //     spot, row_start, row_end, column_start, column_end
            // );
            match spot {
                'B' => row_start += ((row_end - row_start) / 2) + 1,
                'F' => row_end -= ((row_end - row_start) / 2) + 1,
                'R' => column_start += ((column_end - column_start) / 2) + 1,
                'L' => column_end -= ((column_end - column_start) / 2) + 1,
                _ => (),
            }

            // println!(
            //     "plane {} {}, row {} {}, id {}",
            //     row_start, row_end, column_start, column_end, seat_id
            // );
        }
        let seat_id = row_start * 8 + column_start;
        highest = cmp::max(highest, seat_id);
        seat_list.push(seat_id);
    }
    seat_list.sort();
    for i in 0..seat_list.len() - 1 {
        if seat_list[i] + 1 != seat_list[i + 1] {
            println!("part2: seat {}", seat_list[i] + 1)
        }
    }
    highest
}

fn main() {
    let seats = fs::read_to_string("input.txt").expect("error reading file");
    //let seats = fs::read_to_string("test.txt").expect("error reading file");
    let highest = calc_highest(seats);
    println!("part1: {}", highest);
}
