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

struct ReportAnalyzer {
    data_manager: DataManager,
}

impl ReportAnalyzer {
    fn new(data_manager: DataManager) -> Self {
        Self { data_manager }
    }

    fn check_safe_progression(levels: &[i32]) -> bool {
        let mut expected_progression_direction: Option<ProgressDirection> = None;

        let is_safe = levels.windows(2).all(|v_pair| {
            let ord_dir = match v_pair.first().cmp(&v_pair.last()) {
                std::cmp::Ordering::Less => ProgressDirection::Increase,
                std::cmp::Ordering::Equal => ProgressDirection::Incorrect,
                std::cmp::Ordering::Greater => ProgressDirection::Decrease,
            };

            match &expected_progression_direction {
                Some(value) => {
                    if *value != ord_dir {
                        return false;
                    }
                }
                None => expected_progression_direction = Some(ord_dir),
            };

            let diff = v_pair[0] - v_pair[1];

            diff.abs() >= 1 && diff.abs() <= 3
        });

        is_safe
    }

    fn count_safe_reports(&self) -> i32 {
        let mut total_safe_reports = 0;

        for report in &self.data_manager.reports {
            if Self::check_safe_progression(report) {
                total_safe_reports += 1;
            }
        }

        total_safe_reports
    }

    fn count_safe_reports_with_damping(&self) -> i32 {
        let mut total_safe_reports = 0;

        for report in &self.data_manager.reports {
            if Self::check_safe_progression(report) {
                total_safe_reports += 1;
                continue;
            } else {
                for i in 0..report.len() {
                    let mut damped_data = report.clone();
                    damped_data.remove(i);
                    if Self::check_safe_progression(&damped_data) {
                        total_safe_reports += 1;
                        break;
                    }
                }
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

    let total_safe_reports_damped = report_analyer.count_safe_reports_with_damping();
    println!("Total safe damped reports: {}", total_safe_reports_damped);
}