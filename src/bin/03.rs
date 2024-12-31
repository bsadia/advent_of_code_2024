use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use regex::Regex;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_part1: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {

        let lines:Vec<String> = reader.lines().flatten().collect();
        //println!("{:?}", lines);

        let re = Regex::new(r"mul\((\d+),(\d+)\)")?;

        let mut total_sum = 0;

        for line in lines{
            for cap in re.captures_iter(&line){
                    // Extract the two numbers from the capture groups
                    let number1: i32 = cap[1].parse()?;
                    let number2: i32 = cap[2].parse()?;

                    total_sum += number1 * number2;

            }

        }

        println!("Total Sum: {}", total_sum);
        Ok(total_sum)

    }

    assert_eq!(161, part1(BufReader::new(TEST_part1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {

        let lines:Vec<String> = reader.lines().flatten().collect();

        let re_token = Regex::new(r"mul\(\s*\d+\s*,\s*\d+\s*\)|do\(\)|don't\(\)")?;
        let re_mul = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)")?;
        let re_do = Regex::new(r"do\(\)")?;
        let re_dont = Regex::new(r"don't\(\)")?;

        let mut is_enabled = true;
        let mut total_sum = 0;


        for line in lines{

            for token in re_token.find_iter(&line) {
                let token = token.as_str();

                if re_do.is_match(token) {
                    is_enabled = true;


                } else if re_dont.is_match(token) {
                    is_enabled = false;
                }

                if is_enabled {
                    if let Some(cap) = re_mul.captures(token) {
                        let number1: i32 = cap[1].parse()?;
                        let number2: i32 = cap[2].parse()?;
                        total_sum += number1 * number2;
                    }
                }
            }


        }
        println!("Total Sum: {}", total_sum);
        Ok(total_sum)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
