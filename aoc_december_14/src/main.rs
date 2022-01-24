use std::fs::File;
use std::io::{BufReader, BufRead};

struct FoldPoint {
    axis: String,
    coordinate: i32
}

struct Point {
    x: i32,
    y: i32
}

fn main() {
    println!("Hello, world!");

    let file = File::open("./src/inputs/input_aoc_december_14.txt").unwrap();
    let reader = BufReader::new(file);

    let mut folds_vec:Vec<FoldPoint> = Vec::new();
    let mut points_vec:Vec<Point> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line_string = line.unwrap();

        println!("{}. {}", index + 1, line_string);

        if line_string.is_empty() {
            //skip
            continue;
        } else if line_string.contains("fold along") { // THIS LINE IS A FOLD COORDINATE
            let split_line = line_string.split("fold along ");
            let vec_split = split_line.collect::<Vec<&str>>();

            let vec_axis = vec_split[1].split("=").collect::<Vec<&str>>();

            let axis:String = vec_axis[0].to_string();
            let coordinate = vec_axis[1].to_string().parse::<i32>().unwrap();

            // WHY IF I MOVE THE LINE BELOW AFTER THE NEXT ONE, I GET A COMPILE ERROR?

            folds_vec.push(FoldPoint {axis: axis, coordinate: coordinate});
            //println!("FOLD POINT - AXIS = {} - COORDINATE = {}", fold_point.axis, fold_point.coordinate);
        } else { //THIS LINE IS A COORDINATE
            let vec_split = line_string.split(",").collect::<Vec<&str>>();

            let point = Point{x: vec_split[0].to_string().parse::<i32>().unwrap(),
                y: vec_split[1].to_string().parse::<i32>().unwrap()};

            points_vec.push(point);
        }

        // find the max x and max y from the list of points
        // create a matrix of max x and max y

        // if line is empty, skip
        // if line contains "fold along" then store it somehow (maybe list of structs)
    }

    let mut max_point_x = -1;
    let mut max_point_y = -1;

    for point in &points_vec {
        if point.x > max_point_x {
            max_point_x = point.x;
        }

        if point.y > max_point_y {
            max_point_y = point.y;
        }
    }

    println!("MAX POINT X = {}", max_point_x);
    println!("MAX POINT Y = {}", max_point_y);

    let mut matrix = vec![vec![0; (max_point_x + 1) as usize]; (max_point_y + 1) as usize];

    println!("MATRIX 1 = {}", matrix[0][10]);

    for point in &points_vec {
        matrix[point.y as usize][point.x as usize] = 1;
    }

    //println!("MATRIX HURRRDURR = {}", &matrix[1021][700]);


    //START FOLDING THE MATRIX

    for fold_point in &folds_vec {
        let mut number_of_points = 0;

        if fold_point.axis == "x" {

            let mut left_x = fold_point.coordinate - 1;
            let mut right_x = fold_point.coordinate + 1;

            while left_x >= 0 || right_x <= max_point_x {
                for y in 0..((max_point_y + 1) as usize) {
                    let mut found = false;

                    if left_x >= 0 && matrix[y][left_x as usize] == 1 {
                        found = true;
                        number_of_points += 1;
                        println!("POINT FOUND AT X = {} AND Y = {}", left_x, y);
                    }

                    if right_x <= max_point_x && matrix[y][right_x as usize] == 1 {
                        if found == false {
                            number_of_points += 1;
                            println!("POINT FOUND AT X = {} AND Y = {}", right_x, y);
                        }
                    }
                }

                left_x -= 1;
                right_x += 1;
            }

            println!("NUMBER OF POINTS FOR FOLD ALONG X = {} IS = {}", fold_point.coordinate, number_of_points);
            println!("left x = {} and right x = {}", left_x, right_x);
            break;
        } else {
            let mut left_y = fold_point.coordinate - 1;
            let mut right_y = fold_point.coordinate + 1;

            while left_y >= 0 || right_y <= max_point_y {
                for x in 0..((max_point_x + 1) as usize) {
                    let mut found = false;

                    if left_y >= 0 && matrix[left_y as usize][x] == 1 {
                        found = true;
                        number_of_points += 1;
                        println!("POINT FOUND AT X = {} AND Y = {}", x, left_y);
                    }

                    if right_y <= max_point_y && matrix[right_y as usize][x] == 1 {
                        if found == false {
                            number_of_points += 1;
                            println!("POINT FOUND AT X = {} AND Y = {}", x, right_y);
                        }
                    }
                }

                left_y -= 1;
                right_y += 1;
            }

            println!("NUMBER OF POINTS FOR FOLD ALONG Y = {} IS = {}", fold_point.coordinate, number_of_points);
            println!("left y = {} and right y = {}", left_y, right_y);
        }
    }

}
