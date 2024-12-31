use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let blinks = 25;
        let total_stones = count_stones_after_blinks(reader, blinks);

        println!("Total number of stones after {} blinks: {}", blinks, total_stones);

        Ok(total_stones)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    fn count_stones_after_blinks<R: BufRead>(reader: R, blinks: usize) -> usize {

        let  initial_stones: Vec<usize> = reader
            .lines()
            .next()
            .and_then(|line| line.ok())
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|word| word.parse::<usize>().ok())
                    .collect()
            })
            .unwrap_or_default();

        let mut stone_counts: HashMap<usize, usize> = HashMap::new();

        // Initialize the map with the initial stones
        for stone in initial_stones {
            *stone_counts.entry(stone).or_insert(0) += 1;
        }

        // Process each blink
        for _ in 0..blinks {
            let mut new_counts: HashMap<usize, usize> = HashMap::new();

            for (&stone, &count) in stone_counts.iter() {
                if stone == 0 {
                    // Rule 1: Replace `0` with `1`
                    *new_counts.entry(1).or_insert(0) += count;
                } else if stone.to_string().len() % 2 == 0 {
                    // Rule 2: Split stones with even number of digits
                    let digits = stone.to_string();
                    let mid = digits.len() / 2;
                    let left: usize = digits[..mid].parse().unwrap();
                    let right: usize = digits[mid..].parse().unwrap();

                    *new_counts.entry(left).or_insert(0) += count;
                    *new_counts.entry(right).or_insert(0) += count;
                } else {
                    // Rule 3: Multiply by 2024
                    let new_stone = stone * 2024;
                    *new_counts.entry(new_stone).or_insert(0) += count;
                }
            }

            stone_counts = new_counts; // Update stone counts
        }

        // Total number of stones
        stone_counts.values().sum()
    }

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let blinks = 75;
        let total_stones = count_stones_after_blinks(reader, blinks);

        println!("Total number of stones after {} blinks: {}", blinks, total_stones);


        Ok(total_stones)
    }

   // assert_eq!(65601038650482, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
