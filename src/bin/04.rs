use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let grid =
            reader
                .lines()
                .flatten()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

        let word = "XMAS";
        let rows = grid.len();
        let cols = grid[0].len();
        let word_len = word.len();
        let word_chars: Vec<char> = word.chars().collect();
        let mut results = Vec::new();

        // Helper function to check a single direction
        let check_direction = |r: usize, c: usize, dr: isize, dc: isize| -> bool {
            for i in 0..word_len {
                let nr = r as isize + i as isize * dr;
                let nc = c as isize + i as isize * dc;
                if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                    return false;
                }
                if grid[nr as usize][nc as usize] != word_chars[i] {
                    return false;
                }
            }
            true
        };

        // Iterate over every position in the grid
        for r in 0..rows {
            for c in 0..cols {
                // Check all 8 directions
                let directions = [
                    (0, 1),  // Right
                    (0, -1), // Left
                    (1, 0),  // Down
                    (-1, 0), // Up
                    (1, 1),  // Down-right
                    (1, -1), // Down-left
                    (-1, 1), // Up-right
                    (-1, -1), // Up-left
                ];

                for &(dr, dc) in &directions {
                    if check_direction(r, c, dr, dc) {
                        results.push((r, c, format!("Direction: ({}, {})", dr, dc)));
                    }
                }
            }
        }
        println!("Found {} XMAS", results.len());

        Ok(results.len())
    }


    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);


    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let grid: Vec<Vec<char>> = reader
            .lines()
            .flatten()
            .map(|line| line.trim().chars().collect::<Vec<char>>())
            .collect();

        let mut matches = Vec::new();
        let rows = grid.len();
        let cols = grid[0].len();

        for i in 1..rows - 1 {
            for j in 1..cols - 1 {

                if grid[i][j] == 'A' {

                    if (grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S' ||
                        grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M') &&
                        (grid[i + 1][j - 1] == 'M' && grid[i - 1][j + 1] == 'S' ||
                        grid[i + 1][j - 1] == 'S' && grid[i - 1][j + 1] == 'M')
                    {
                        matches.push((i, j));
                    }

                }
            }
        }


        println!("Found {} X-MAS", matches.len());

        Ok(matches.len())
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
   // endregion

    Ok(())
}
