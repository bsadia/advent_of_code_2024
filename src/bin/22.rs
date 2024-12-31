use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "22";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1
10
100
2024
";

const TEST2: &str = "\
1
2
3
2024
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn compute(mut num:u64, limit:i64) -> u64 {

        for _ in 0..limit {
            num = (num ^ (num * 64)) % 16_777_216;
            num = (num ^ (num / 32)) % 16_777_216;
            num = (num ^ (num * 2048)) % 16_777_216;
        }
        num

    }

    fn part1<R: BufRead>(reader: R) -> Result<u64> {

       let data: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
        let mut result = 0;

        for d in data.clone(){
            let num =  d.to_string().parse::<u64>()?;
            let y = compute(num, 2000);
            result +=y;
        }

        println!("Sum of 2000th Secret number:{}", result);

        Ok(result)
    }
    // 1: 8685429
    // 10: 4700978
    // 100: 15273692
    // 2024: 8667524

    assert_eq!(37327623, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn get_sequence(mut x: u32, r: usize) -> (u32, Vec<u32>, Vec<i32>) {
        let mut seq = Vec::new();
        let mut diff = Vec::new();

        for _ in 0..r {
            let a = x % 10;

            // Use u64 for intermediate calculations to prevent overflow
            x = ((x as u64 ^ (x as u64 * 64)) % 16_777_216) as u32;
            x = ((x as u64 ^ (x as u64 / 32)) % 16_777_216) as u32;
            x = ((x as u64 ^ (x as u64 * 2048)) % 16_777_216) as u32;

            let current = x % 10;
            diff.push((current as i32) - (a as i32));
            seq.push(current);
        }
            (x, seq, diff)
    }


    fn part2<R: BufRead>(reader: R) -> Result<u32> {

        let lines: Vec<u32> = reader
            .lines()
            .map(|line| line.unwrap().trim().parse().unwrap())
            .collect();

        let mut dic: HashMap<Vec<i32>, u32> = HashMap::new();

        for &x in &lines {
            let (_, seq, diff_values) = get_sequence(x, 2000);
            let mut seen: HashSet<Vec<i32>> = HashSet::new();

            for i in 0..diff_values.len().saturating_sub(3) {
                let pat: Vec<i32> = diff_values[i..i + 4].to_vec();
                let point = seq.get(i + 3).copied().unwrap_or(0);

                if !seen.contains(&pat) {
                    seen.insert(pat.clone());
                    *dic.entry(pat).or_insert(0) += point;
                }
            }
        }

        let max_value = dic.values().max().copied().unwrap_or(0);
        println!("{}", max_value);
        Ok(max_value)
    }

    assert_eq!(23, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
