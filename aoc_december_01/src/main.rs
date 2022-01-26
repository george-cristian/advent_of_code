use std::fs::File;
use std::io::{BufReader, BufRead};

const WINDOW_SIZE:i32 = 3;

fn main() {

    let input_file = File::open("./src/inputs/input_aoc_december_01.txt").unwrap();
    let input_reader = BufReader::new(input_file);

    let mut nr_of_increases = 0;

    let mut nr_of_measurements = 1;

    let mut first_window = 0;
    let mut second_window = 0;
    let mut third_window = 0;

    for (_index, line) in input_reader.lines().enumerate() {

        let line_string = line.unwrap();

        let current_measurement = line_string.to_string().parse::<i32>().unwrap();

        if nr_of_measurements <= WINDOW_SIZE {

            if nr_of_measurements % WINDOW_SIZE == 1 {
                first_window = first_window + current_measurement;
            } else if nr_of_measurements % WINDOW_SIZE == 2 {
                first_window  = first_window + current_measurement;
                second_window = second_window + current_measurement;
            } else {
                first_window  = first_window + current_measurement;
                second_window = second_window + current_measurement;
                third_window  = third_window + current_measurement;
            }

        } else {
            second_window = second_window + current_measurement;

            if second_window > first_window {
                nr_of_increases += 1;
            }

            first_window  = second_window;
            second_window = third_window + current_measurement;
            third_window  = current_measurement;
        }

        nr_of_measurements += 1;
    }

    println!("Total number of increases: {}", nr_of_increases);

}


// PART 1
/*let input_file = File::open("./src/inputs/input_aoc_december_01.txt").unwrap();
let input_reader = BufReader::new(input_file);

let mut previous_measurement:i32 = -1;
let mut nr_of_increases = 0;

for (_index, line) in input_reader.lines().enumerate() {

let line_string = line.unwrap();

let current_measurement = line_string.to_string().parse::<i32>().unwrap();

if previous_measurement != -1 && current_measurement > previous_measurement {
nr_of_increases += 1;
}

previous_measurement = current_measurement;
}

println!("Total number of increases: {}", nr_of_increases);*/