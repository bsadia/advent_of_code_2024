use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "25";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        // Read lines efficiently and split raw data into sections
        let raw_data: String = reader.lines().filter_map(Result::ok).collect::<Vec<_>>().join("\n");
        let sections: Vec<&str> = raw_data.split("\n\n").collect();

        // Extract dimensions
        let width = sections[0].lines().next().unwrap().len();
        let height = sections[0].lines().count();

        let mut keys = Vec::new();
        let mut locks = Vec::new();

        // Process each section
        for section in sections {
            let mut pin_count = vec![-1; width];
            let mut is_lock = false;

            // Process pins directly without collecting into a Vec<char>
            for (i, ch) in section.chars().filter(|&c| c != '\n').enumerate() {
                if ch == '#' {
                    pin_count[i % width] += 1;
                    // Check if this section represents a lock
                    if i == 0 {
                        is_lock = true;
                    }
                }
            }

            // Categorize as keys or locks
            if is_lock {
                locks.push(pin_count);
            } else {
                keys.push(pin_count);
            }
        }

        let mut count = 0;

        for key in &keys {
            for lock in &locks {

                if key.iter()
                    .zip(lock.iter())
                    .all(|(&k, &l)| k + l <= (height as i32 - 2))
                {
                    count += 1;
                }
            }
        }

        println!("{}", count);
        Ok(count)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}


