use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    let input_file = File::open("./src/inputs/input_aoc_december_01.txt").unwrap();
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

    println!("Total number of increases: {}", nr_of_increases);

}
