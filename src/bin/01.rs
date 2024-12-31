use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::collections::HashMap;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);
    fn parse_input(lines:Vec<String>) -> (Vec<i32>, Vec<i32>) {

        let mut left_column: Vec<i32> = Vec::new();
        let mut right_column: Vec<i32> = Vec::new();

        for line in lines {

            let numbers: Vec<i32> = line
                .split_whitespace() // Split the line by whitespace
                .map(|x| x.parse::<i32>().unwrap()) // Parse each split part as an integer
                .collect();
            left_column.push(numbers[0]);
            right_column.push(numbers[1]);
        }
        (left_column, right_column)
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {


        let lines: Vec<String> = reader.lines().flatten().collect();
        let (mut left_column, mut right_column) = parse_input(lines);

        left_column.sort_unstable();
        right_column.sort_unstable();

        let total_distance: i32 = left_column
            .iter()
            .zip(right_column.iter())
            .map(|(left, right)| (left - right).abs())
            .sum();

        println!("Total distance: {}", total_distance);
        Ok(total_distance)

    }


    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result); //3246517
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let lines: Vec<String> = reader.lines().flatten().collect();
        let (left_column, right_column) = parse_input(lines);

        let right_counts = count_occurrences(&right_column);

        let mut total_sum = 0;
        for &num in &left_column {
            let count_in_right = right_counts.get(&num).cloned().unwrap_or(0);
            total_sum += num * count_in_right;
        }

        println!("Total sum: {}", total_sum);

        Ok(total_sum)
    }

    fn count_occurrences(vec: &[i32]) -> HashMap<i32, i32> {
        let mut counts = HashMap::new();
        for &num in vec {
            *counts.entry(num).or_insert(0) += 1;
        }
        counts
    }


    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result); //29379307
    //endregion

    Ok(())
}
