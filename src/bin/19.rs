use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn get_designs(pattern: &str, towel_list: &[String], cache: &mut HashMap<String, bool>) -> bool {
        if let Some(&cached_result) = cache.get(pattern) {
            return cached_result;
        }

        if pattern.is_empty() {
            cache.insert(pattern.to_string(), true);
            return true;
        }

        for towel in towel_list {
           // eprintln!("Checking: design = '{}', option = '{:?}'", pattern, towel);
            if pattern.starts_with(towel) && get_designs(&pattern[towel.len()..], towel_list, cache) {
                cache.insert(pattern.to_string(), true);
                return true;
            }
        }

        cache.insert(pattern.to_string(), false);
        false
    }


    fn parse_input<R: BufRead>(reader: R) -> (Vec<String>, Vec<String>) {
        let mut lines = reader.lines();

        let binding = lines.next().unwrap().unwrap();
        let towel_list: Vec<String> = binding.split(',')
            .map(|s| s.trim().trim_matches('"').to_string())
            .collect();
        lines.next();

        // Read desired patterns
        let desired_pattern: Vec<String> = lines.map(|line| line.unwrap()).collect();
       ( towel_list,  desired_pattern)

    }


    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let (towel_list, desired_pattern) = parse_input(reader);

        let mut output = 0;
        let mut cache = HashMap::new();

        for pattern in desired_pattern {
            if get_designs(&pattern, &towel_list, &mut cache) {
                output += 1;
            }
        }
        println!("{}", output);
        Ok(output)
    }


    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn get_all_possible_designs(pattern: &str, towel_list: &[String], cache: &mut HashMap<String, i64>) -> i64 {
        if let Some(&cached_result) = cache.get(pattern) {
            return cached_result;
        }


        if pattern.is_empty() {
            cache.insert(pattern.to_string(), 1);
            return 1;
        }
        let mut count = 0;

        for towel in towel_list {
            if pattern.starts_with(towel) {
                count += get_all_possible_designs(&pattern[towel.len()..], towel_list, cache);
            }
        }

        cache.insert(pattern.to_string(), count);
        count
    }

    fn part2<R: BufRead>(reader: R) -> Result<i64> {

        let (towel_list, desired_pattern) = parse_input(reader);

        let mut output: i64 = 0;
        let mut cache = HashMap::new();

        for pattern in desired_pattern {
            output += get_all_possible_designs(&pattern, &towel_list, &mut cache) ;
        }

        println!("{}", output);
        Ok(output)
    }

    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
