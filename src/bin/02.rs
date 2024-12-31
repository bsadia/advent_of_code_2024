use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);


    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let lines: Vec<String> = reader.lines().flatten().collect();

        let mut counter = 0;

        for  line in lines {
            // println!("{}", line);
            let report: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            // Check if the report is safe
            if is_safe_report(&report) {
                counter += 1
            }
        }

        Ok(counter)

    }
    // Function to check if a report is safe
    fn is_safe_report(report: &[i32]) -> bool {

        let is_increasing = report.windows(2).all(|pair| pair[0] < pair[1]);
        let is_decreasing = report.windows(2).all(|pair| pair[0] > pair[1]);

        // Check if adjacent levels differ by at least 1 and at most 3
        let valid_differences = report.windows(2).all(|pair| {
            let diff = (pair[0] - pair[1]).abs();
            diff >= 1 && diff <= 3
        });

        // The report is safe if both conditions are met
        (is_increasing || is_decreasing) && valid_differences
    }


    fn is_safe_or_can_be_made_safe(report: &[i32]) -> bool {
        // Check if the report is already safe
        if is_safe_report(report) {
            return true;
        }

        // Attempt to remove one element and check if the resulting report is safe
        for i in 0..report.len() {
            let mut modified_report = report.to_vec();
            modified_report.remove(i); // Remove the ith element
            if is_safe_report(&modified_report) {
                return true;
            }
        }

        // If no single removal makes it safe, return false
        false
    }


    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
       let lines: Vec<String> = reader.lines().flatten().collect();
        let mut counter = 0;

        for  line in lines{
            // println!("{}", line);

            let report: Vec<i32> = line
                .split_whitespace() // Split the line into numbers
                .map(|x| x.parse::<i32>().unwrap()) // Parse each number
                .collect();

            // Check if the report is safe
            if is_safe_or_can_be_made_safe(&report) {
                // println!("Safe report found at line {}: {:?}", index + 1, report);
                counter += 1
            }
        }

        Ok(counter)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
