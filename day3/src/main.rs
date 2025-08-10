use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

struct DataManager {
    data: Vec<String>,
}

impl DataManager {
    fn parse_input(input_file: &str) -> Self {
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);

        let lines = reader.lines();

        let mut data: Vec<String> = Vec::new();
        for line in lines {
            let line = line.unwrap();
            data.push(line);
        }

        Self { data }
    }
}

struct Multiplier {
    data_manager: DataManager,
}

impl Multiplier {
    fn new(data_manager: DataManager) -> Self {
        Self { data_manager }
    }

    fn extract_operations(input_data: &Vec<String>) -> Vec<(i32, i32)> {
        //mul\([0-9]{1,3},[0-9]{1,3}\)
        let re = Regex::new(r"mul\((?<term1>[0-9]{1,3}),(?<term2>[0-9]{1,3})\)").unwrap();

        let mut operation_terms: Vec<(i32, i32)> = Vec::new();

        for line in input_data {
            for cap in re.captures_iter(&line) {
                let (_, [term1, term2]) = cap.extract();
                operation_terms
                    .push((term1.parse::<i32>().unwrap(), term2.parse::<i32>().unwrap()));
            }
        }

        operation_terms
    }

    fn compute_result(&self) -> i32 {
        let operations = Self::extract_operations(&self.data_manager.data);

        let mut result = 0;
        for (term1, term2) in operations {
            result += term1 * term2;
        }

        result
    }
}

fn main() {
    let data_manager = DataManager::parse_input("day3/data/input.txt");

    let multiplier = Multiplier::new(data_manager);

    let result = multiplier.compute_result();
    println!("Result 1: {result}");
}
