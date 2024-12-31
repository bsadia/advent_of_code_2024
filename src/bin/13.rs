use std::result::Result::Ok;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");



    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let mut tokens = 0;
        let mut problem: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.trim().is_empty() {
                if !problem.is_empty() {
                    // Solve the problem for the current machine
                    tokens += solve(&problem[0], &problem[1], &problem[2], false);
                    problem.clear();
                }
            } else {
                problem.push(line);
            }
        }

        // Solve the last problem if any
        if !problem.is_empty() {
            tokens += solve(&problem[0], &problem[1], &problem[2], false);
        }

        println!("Tokens required {}", tokens);
        Ok(tokens)
    }


    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
       // let machines = parse_machines(reader, false);

        let mut tokens = 0;
        let mut problem: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.trim().is_empty() {
                if !problem.is_empty() {
                    // Solve the problem for the current machine
                    tokens += solve(&problem[0], &problem[1], &problem[2], true);
                    problem.clear();
                }
            } else {
                problem.push(line);
            }
        }

        // Solve the last problem if any
        if !problem.is_empty() {
            tokens += solve(&problem[0], &problem[1], &problem[2], true);
        }

        println!("Tokens required {}", tokens);

        Ok(tokens)
    }
    fn parse_line(line: &str, delimiter: char) -> (isize, isize) {
        let parts: Vec<&str> = line.split(':').collect();
        let coords: Vec<&str> = parts[1].trim().split(',').collect();
        let x = coords[0].split(delimiter).nth(1).unwrap().trim().parse::<isize>().unwrap();
        let y = coords[1].split(delimiter).nth(1).unwrap().trim().parse::<isize>().unwrap();
        (x, y)
    }


    fn solve(la: &str, lb: &str, lp: &str, part_2: bool) -> usize {
        let (xa, ya) = parse_line(la, '+');
        let (xb, yb) = parse_line(lb, '+');
        let (mut xp, mut yp) = parse_line(lp, '=');

        // Adjust prize coordinates for part 2
        if part_2 {
            let adjustment = 10_000_000_000_000;
            xp += adjustment;
            yp += adjustment;
        }

        // Denominator for solving the linear equations
        let denominator = xa * yb - ya * xb;
        if denominator == 0 {
            return 0; // Parallel lines, no intersection
        }

        // Calculate B
        let b_numerator = yp * xa - ya * xp;
        if b_numerator % denominator != 0 {
            return 0; // B must be an integer
        }
        let b = b_numerator / denominator;
        if b < 0 {
            return 0; // B must be non-negative
        }

        // Calculate A
        let a_numerator = xp - xb * b;
        if a_numerator % xa != 0 {
            return 0; // A must be an integer
        }
        let a = a_numerator / xa;
        if a < 0 {
            return 0; // A must be non-negative
        }

        // Return total cost
        (a * 3 + b) as usize
    }

    assert_eq!(875318608908, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
