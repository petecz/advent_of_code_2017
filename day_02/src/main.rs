use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::u32;

fn open_input_file() -> BufReader<File> {
    let path = Path::new("./src/input.txt");
    let f = File::open(&path).expect("File not found");
    let f = BufReader::new(f);

    f
}

fn part_1() -> u32 {
    let f = open_input_file();
    let mut checksum: u32 = 0;

    for line in f.lines().filter_map(|result| result.ok()) {
        let vec: Vec<&str> = line.split("\t").collect();
        let mut max: u32 = 0;
        let mut min: u32 = u32::MAX;
        for current_number in vec {
            let n: u32 = current_number.parse().unwrap();
            if n > max { max = n; }
            if n < min { min = n; }
        }
        checksum += max - min;
    }

    checksum
}

fn part_2() -> u32 {
    let f = open_input_file();
    let mut sum: u32 = 0;

    for line in f.lines().filter_map(|result| result.ok()) {
        let vec: Vec<&str> = line.split("\t").collect();
        let mut result: u32 = 0;
        for current_number in &vec {
            let n: u32 = current_number.parse().unwrap();
            for m in &vec {
                let m: u32 = m.parse().unwrap();
                if n != m && n % m == 0 {
                    result = n / m;
                }
                if n != m && m % n == 0 {
                    result = m / n;
                }
            }
            if result != 0 {continue}
        }
        sum += result;
    }

    sum
}

fn main() {
    let result = part_1();
    let result_2 = part_2();
    println!("Part 1: {}, Part 2: {}", result, result_2);
}
