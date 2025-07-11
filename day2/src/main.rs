use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct DataManager {
    reports: Vec<Vec<i32>>,
}

impl DataManager {
    fn read_input(file_name: &str) -> Self {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        let lines = reader.lines();

        let mut reports: Vec<Vec<i32>> = Vec::new();
        for line in lines {
            let line = line.unwrap();
            let values: Vec<&str> = line.split(" ").collect();
            let mut levels: Vec<i32> = Vec::new();
            for v in values {
                levels.push(v.parse::<i32>().unwrap());
            }
            reports.push(levels);
        }

        Self { reports }
    }
}

#[derive(PartialEq)]
enum ProgressDirection {
    Increase,
    Decrease,
    Incorrect,
}

impl ProgressDirection {
    fn get_order_direction_from_diff(diff: i32) -> Self {
        let order_dir = match diff {
            i if i < 0 => ProgressDirection::Increase,
            d if d > 0 => ProgressDirection::Decrease,
            z if z == 0 => ProgressDirection::Incorrect,
            _ => ProgressDirection::Incorrect,
        };
        order_dir
    }
}

struct ReportAnalyzer {
    data_manager: DataManager,
}

impl ReportAnalyzer {
    fn new(data_manager: DataManager) -> Self {
        Self { data_manager }
    }

    fn check_safe_progression(levels: &Vec<i32>) -> bool {
        let expected_progression_direction = ProgressDirection::get_order_direction_from_diff(
            levels.first().unwrap() - levels.last().unwrap(),
        );
        let is_safe = levels.windows(2).all(|v_pair| {
            let diff = v_pair[0] - v_pair[1];
            let ord_dir = ProgressDirection::get_order_direction_from_diff(diff);

            ord_dir == expected_progression_direction && diff.abs() >= 1 && diff.abs() <= 3
        });

        is_safe
    }

    fn count_safe_reports(&self) -> i32 {
        let mut total_safe_reports = 0;

        for report in &self.data_manager.reports {
            if Self::check_safe_progression(&report) {
                total_safe_reports += 1;
            }
        }

        total_safe_reports
    }
}

fn main() {
    let file_name = "day2/data/input.txt";

    let data_manager: DataManager = DataManager::read_input(file_name);
    let report_analyer = ReportAnalyzer::new(data_manager);

    let total_safe_reports = report_analyer.count_safe_reports();

    println!("Total safe reports: {}", total_safe_reports);
}
