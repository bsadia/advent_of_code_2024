use std::collections::{HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_map<R: BufRead>(reader: R) -> Result<Vec<Vec<u8>>> {
        let map:Vec<Vec<u8>>  =
            reader
                .lines()
                .map(|line| {
                    line.unwrap()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect();
        Ok(map)
    }


    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let map:Vec<Vec<u8>>  = parse_map(reader)?;

        let result = compute_trailhead_metrics(map, true);
        println!("Sum of all trailhead scores: {}", result);
        Ok(result)
    }


    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion


    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map:Vec<Vec<u8>>  = parse_map(reader)?;

        let result = compute_trailhead_metrics(map, false);

        println!("Sum of all trailhead scores: {}", result);
        Ok(result)
    }


    fn dfs(
        map: &Vec<Vec<u8>>,
        visited: &mut HashSet<(usize, usize, u8)>,
        x: usize,
        y: usize,
        current_height: u8,
        scores: &mut HashSet<(usize, usize)>,
    ) -> usize {
        let rows = map.len();
        let cols = map[0].len();
        let mut distinct_trails = 0;

        // If we reach height 9, it's a valid trail
        if current_height == 9 {
            scores.insert((x, y)); // Add this position to reachable 9s
            return 1;
        }

        // Possible moves (up, down, left, right)
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < rows && (ny as usize) < cols {
                let nx = nx as usize;
                let ny = ny as usize;

                if map[nx][ny] == current_height + 1
                    && !visited.contains(&(nx, ny, map[nx][ny]))
                {
                    // Mark this cell as visited for the current trail
                    visited.insert((nx, ny, map[nx][ny]));

                    // Perform DFS
                    distinct_trails += dfs(map, visited, nx, ny, map[nx][ny], scores);

                    // Unmark the cell for other trails
                    visited.remove(&(nx, ny, map[nx][ny]));
                }
            }
        }

        distinct_trails
    }

    fn compute_trailhead_metrics(map: Vec<Vec<u8>>, part_1:bool) -> usize {
        let rows = map.len();
        let cols = map[0].len();
        let mut total_score = 0;
        let mut total_rating = 0;

        // Helper function for DFS

        // Loop through all cells to find trailheads
        for i in 0..rows {
            for j in 0..cols {
                if map[i][j] == 0 {
                    let mut visited = HashSet::new();
                    let mut scores = HashSet::new();
                    let trails_from_here = dfs(&map, &mut visited, i, j, 0, &mut scores);

                    total_score += scores.len(); // Add number of reachable 9s
                    total_rating += trails_from_here; // Add number of distinct trails
                }
            }
        }
        if part_1{
            total_score
        }
        else { total_rating}


    }



    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
