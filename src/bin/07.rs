use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {

        let  total_test_value = process_input(BufReader::new(reader), 2);

        println!("Sum of valid equations: {}", total_test_value);

        Ok(total_test_value)
    }


    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    fn process_input<R: BufRead>(reader: R, operators:i32) ->  i64 {
        let mut valid_equations = Vec::new();

        let mut total_test_value = 0;

        for line in reader.lines().flatten() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                continue;
            }


            let test_value: i64 = parts[0].trim().parse().unwrap_or(-1);
            let numbers: Vec<i64> = parts[1]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();


            // Evaluate all possible combinations of + and * operators
            if let Some(equation) = find_valid_equation(test_value, &numbers, operators) {
                valid_equations.push(equation);
                total_test_value += test_value;
            }

        }

        total_test_value

    }


    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let  total_test_value = process_input(BufReader::new(reader), 3);


        println!("Sum of valid equations: {}", total_test_value);
        Ok(total_test_value)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion
    fn find_valid_equation(test_value: i64, numbers: &[i64], operators: i32) -> Option<String> {
        let n = numbers.len();
        let num_operators = n - 1;

        // Generate all combinations of '+', '*', and '||' operators
        // processed bitwise to determine the operator at each position
        for mask in 0..operators.pow(num_operators as u32) {
            let mut result = numbers[0];
            let mut equation = format!("{}", numbers[0]);

            let mut current_mask = mask;
            for (_, &num) in numbers.iter().enumerate().skip(1) {
                let operator = current_mask % operators; // Extract the operator (0 = +, 1 = *, 2 = ||)
                current_mask /= operators;

                match operator {
                    0 => {
                        result += num;
                        equation.push_str(&format!(" + {}", num));
                    }
                    1 => {
                        result *= num;
                        equation.push_str(&format!(" * {}", num));
                    }
                    2 => {
                        let concatenated = concat_numbers(result, num);
                        equation = format!("{} || {}", equation, num);
                        result = concatenated;
                    }
                    _ => unreachable!(),
                }
            }

            if result == test_value {
                return Some(format!("{} = {}", equation, test_value));
            }

        }

        None
    }

    fn concat_numbers(left: i64, right: i64) -> i64 {
        let right_digits = count_digits(right);
        left * 10_i64.pow(right_digits) + right
    }

    fn count_digits(mut num: i64) -> u32 {
        let mut count = 0;
        while num > 0 {
            count += 1;
            num /= 10;
        }
        count
    }
    Ok(())
}
