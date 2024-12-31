use std::collections::VecDeque;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_PART1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
const TEST: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    fn parse_input<R: BufRead>(reader: R) -> (i64, i64, i64, Vec<i64>) {

        let lines: Vec<String> = reader
            .lines()
            .filter_map(Result::ok)
            .collect();

        let blank_line_index = lines.iter().position(|line| line.trim().is_empty()).unwrap();

        let registers_part = &lines[..blank_line_index];
        let program_part = &lines[blank_line_index + 1..];


        let a:i64 = registers_part[0].split(':').nth(1).unwrap().trim().to_string().parse::<i64>().expect("Invalid program command");
        let b :i64= registers_part[1].split(':').nth(1).unwrap().trim().to_string().parse::<i64>().expect("Invalid program command");
        let c:i64 = registers_part[2].split(':').nth(1).unwrap().trim().to_string().parse::<i64>().expect("Invalid program command");

        let binding = program_part[0].split(':').nth(1).unwrap().trim().to_string();
        let program: Vec<i64> = binding.split(',').map(|x| x.parse().expect("Invalid program command")).collect();

        (a, b, c, program)
    }

    fn combo(operand: i64, a: i64, b: i64, c: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Invalid operand!"),
        }
    }


    fn simulate(code: &[i64], mut a: i64, mut b: i64, mut c: i64, output: &mut Vec<i64>) {
        let mut pc: usize = 0;

        while pc < code.len() {
            let command = code[pc];
            let operand = code[pc + 1];

            let value = match command {
                0 | 2 | 6 | 7 | 5 => Some(combo(operand, a, b, c)), // Commands that use combo
                _ => None, // Other commands don't need combo
            };

            match command {
                0 => { // adv
                    a /= 2_i64.pow(value.unwrap_or(0) as u32);
                }
                1 => { // bxl
                    b ^= operand;
                }
                2 => { // bst
                    b = value.unwrap_or(0) % 8;
                }
                3 => { // jnz
                    if a != 0 {
                        pc = operand as usize;
                        continue;
                    }
                }
                4 => { // bxc
                    b ^= c;
                }
                5 => { // out
                    output.push(value.unwrap_or(0) % 8);
                }
                6 => { // bdv
                    b = a / 2_i64.pow(value.unwrap_or(0) as u32);
                }
                7 => { // cdv
                    c = a / 2_i64.pow(value.unwrap_or(0) as u32);
                }
                _ => {
                    panic!("Invalid command encountered!");
                }
            }

            pc += 2;
        }
    }


    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let (a, b, c, program )= parse_input(reader);

        println!("Register: {}, {}, {}", a , b , c);
        println!("Program: {:?}", program);

        // Initialize output vector
        let mut output = Vec::new();

        // Simulate the program
        simulate(&program, a, b, c, &mut output);

        // Print the output
        let result = output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("Final Output: {}", result);

        Ok(0)
    }


    // Test Answer: 4,6,3,5,6,3,5,2,1,0
    assert_eq!(0, part1(BufReader::new(TEST_PART1.as_bytes()))?);


    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let (a, b, c, program )= parse_input(reader);

        println!("Register: {}, {}, {}", a , b , c);
        println!("Program: {:?}", program);

        let Some(min_value) = get_new_a(b, c, program) else { todo!() };

        Ok(min_value)
    }

    fn get_new_a(b: i64, c: i64, program: Vec<i64>) -> Option<i64> {
        let mut q: VecDeque<i64> = VecDeque::new();
        q.push_back(0);
        let program_len = program.len();

        for i in 0..program_len {
            // Calculate the expected output slice once
            let expected = &program[program_len.saturating_sub(i + 1)..];

            for _ in 0..q.len() {
                if let Some(v) = q.pop_front() {
                    for k in 0..8 {
                        let a = 8 * v + k;

                        // Simulate the output only when necessary
                        let mut output = Vec::with_capacity(expected.len());
                        simulate(&program, a, b, c, &mut output);

                        // Check only the portion of `output` that matches `expected`
                        if output.len() >= expected.len() && &output[output.len() - expected.len()..] == expected {
                            q.push_back(a);
                        }
                    }
                }
            }

            // If there are no candidates left, stop early
            if q.is_empty() {
                break;
            }
        }
        // Return the smallest valid `a` found, or `None` if no valid `a` exists
        q.iter().min().cloned()
    }


    assert_eq!(117440, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
