use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

struct Lists(Vec<i32>, Vec<i32>);

impl Lists {
    fn new() -> Lists {
        Lists(Vec::new(), Vec::new())
    }

    fn sort_lists(&mut self) {
        self.0.sort();
        self.1.sort();
    }

    fn compute_diff(&self) -> i32 {
        let result: Vec<i32> = zip(&self.0, &self.1)
            .map(|(e1, e2)| (e1 - e2).abs())
            .collect();

        result.iter().sum()
    }

    fn compute_similarity(&self) -> usize {
        let result = self.0.iter().fold(0, |acc, e1| {
            acc + *e1 as usize * self.1.iter().filter(|&e2| e2 == e1).count()
        });
        result
    }

    fn frequency_count_in_list2(&self, element: &i32) -> i32 {
        self.1.iter().filter(|&e| e == element).count() as i32
    }

    fn compute_similarity2(&self) -> i32 {
        let inspected: HashMap<i32, i32> = HashMap::new();

        let result = self.0.iter().fold(0, |acc, e| {
            acc + match inspected.get(e) {
                Some(v) => e * v,
                None => match self.frequency_count_in_list2(e) {
                    0 => 0,
                    y => e * y,
                },
            }
        });
        result
    }
}

fn read_file(file_name: &str) -> Lists {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines();

    let mut lists = Lists::new();
    for line in lines {
        let line = line.unwrap();
        let e: Vec<&str> = line.split("   ").collect();
        lists.0.push(e[0].parse().unwrap());
        lists.1.push(e[1].parse().unwrap());
    }
    lists
}

fn main() {
    let mut data = read_file("day1/data/input.txt");

    data.sort_lists();

    let result = data.compute_diff();

    println!("Day 1-1: {}", result);

    // let result = data.compute_similarity();
    let result = data.compute_similarity2();
    println!("Day 1-2: {}", result);
}
