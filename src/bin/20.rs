use std::collections::{HashMap};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

// Directions for movement
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn main() -> Result<()> {
    start_day(DAY);

    // Check if the position (x, y) is valid
    fn valid(x: isize, y: isize, mat: &Vec<Vec<char>>) -> bool {
        x >= 0 && x < mat.len() as isize && y >= 0 && y < mat[x as usize].len() as isize && mat[x as usize][y as usize] != '#'
    }

    // Walk from `start` to `end`, calculate distances, and store in `dist`

    fn walk(
        start: (usize, usize),
        end: (usize, usize),
        mat: &Vec<Vec<char>>,
    ) ->  HashMap<(usize, usize), usize> {

        let mut dist = HashMap::new();
        let mut current = end;
        let mut previous = None;
        let mut c = 0;

        while current != start {
            // Update the distance map only if not already set
            dist.entry(current).or_insert(c);

            c += 1;

            // Find the next valid point to move to
            let mut found_next = false;
            for &(dx, dy) in &DIRECTIONS {
                let next = (
                    (current.0 as isize + dx) as usize,
                    (current.1 as isize + dy) as usize,
                );

                if Some(next) != previous && valid(next.0 as isize, next.1 as isize, mat) {
                    previous = Some(current);
                    current = next;
                    found_next = true;
                    break;
                }
            }

            // If no valid move is found, terminate early (potential invalid input)
            if !found_next {
                panic!("No valid path found during walk!");
            }
        }

        // Update the start point
        dist.entry(start).or_insert(c);
        dist
    }


    //region Part 1
    println!("=== Part 1 ===");

    fn parse_input<R: BufRead>(reader: R) -> HashMap<(usize, usize), usize>{

        let mat: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();

        let mut start = (0, 0);
        let mut end = (0, 0);

        // Find the start ('S') and end ('E') positions
        for (i, row) in mat.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 'S' {
                    start = (i, j);
                } else if cell == 'E' {
                    end = (i, j);
                }
            }
        }
        // Calculate distances
        let dist =  walk(start, end, &mat);
        dist
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {


        let  dist= parse_input(reader);

        let mut cheat_count = 0;

        // Collect and sort keys by their values in descending order
        let mut keys: Vec<(&(usize, usize), &usize)> = dist.iter().collect();
        keys.sort_by(|&(_, &v1), &(_, &v2)| v2.cmp(&v1)); // Sort by distance in descending order

        for (&k, &v) in keys {
            for &(dx, dy) in &DIRECTIONS {
                let p = (
                    (k.0 as isize + 2 * dx) as usize,
                    (k.1 as isize + 2 * dy) as usize,
                );
                if let Some(&nv) = dist.get(&p) {
                    if (v as isize - nv as isize - 2) >= 100 { // Adjust condition as needed for example input
                        cheat_count += 1;
                    }
                }
            }
        }


        println!("{}", cheat_count);
        Ok(0)
    }

    assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion


    //region Part 2
    println!("\n=== Part 2 ===");
    // Manhattan distance heuristic
    fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
        (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let  dist= parse_input(reader);

        let mut cheat_count = 0;

        // Collect keys and sort them in descending order based on their values
        let mut keys: Vec<(&(usize, usize), &usize)> = dist.iter().collect();
        keys.sort_by(|&(_, &v1), &(_, &v2)| v2.cmp(&v1)); // Sort by distance values in descending order

        for (&k, &v) in keys {
            for (&k2, &v2) in dist.iter() {
                // Compute Manhattan distance and filter candidates early
                let d = manhattan(k, k2);
                if d > 20 {
                    continue; // Skip pairs that are too far
                }

                if v2 as isize - v as isize - d as isize >= 100 { // for the test change 100 to 50 and add all the cheat distances
                    cheat_count += 1;
                }
            }
        }

        println!("{}", cheat_count);
        Ok(0)
    }


    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
