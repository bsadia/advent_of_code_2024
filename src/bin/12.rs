use std::collections::{HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
// Directions representing right, down, left, and up neighbors
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];


fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_map<R: BufRead>(reader: R) -> Vec<Vec<char>> {

        reader.lines()
                .flatten()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()

    }


    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let grid = parse_map(reader);

        //println!("{:?}", grid);
        let total_price = calculate_total_price(grid);
        println!("Total price for fencing: {}", total_price);

        Ok(total_price)
    }

    fn flood_fill(
        grid: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        start_row: usize,
        start_col: usize,
        directions: &[(isize, isize)],
    ) -> (usize, usize, usize) {
        let mut stack = vec![(start_row, start_col)];
        let mut area = 0;
        let mut perimeter = 0;
        let plant_type = grid[start_row][start_col];

        let mut region:HashSet<(isize, isize)> = HashSet::new();

        while let Some((row, col)) = stack.pop() {
            if visited[row][col] {
                continue;
            }
            visited[row][col] = true;
            area += 1;
            region.insert((row as isize, col as isize));

            // Check neighbors
            for &(dx, dy) in directions.iter() {
                let new_row = row as isize + dx;
                let new_col = col as isize + dy;

                if new_row >= 0 && new_row < grid.len() as isize && new_col >= 0 && new_col < grid[0].len() as isize {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if grid[new_row][new_col] == plant_type && !visited[new_row][new_col] {
                        stack.push((new_row, new_col));
                    } else if grid[new_row][new_col] != plant_type {
                        // Neighboring cell belongs to another region, contributes to perimeter
                        perimeter += 1;
                    }
                } else {
                    // Out-of-bounds neighbor contributes to perimeter
                    perimeter += 1;
                }
            }
        }
        // println!("{:?}", region);
        let sides = find_sides(&region);

        (area, perimeter, sides)
    }

    fn calculate_total_price(grid: Vec<Vec<char>>) -> usize {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut total_price = 0;

        // Iterate through the grid
        for row in 0..rows {
            for col in 0..cols {
                if !visited[row][col] {
                    let (area, perimeter, _) = flood_fill(&grid, &mut visited, row, col, &DIRECTIONS);
                    total_price += area * perimeter;
                }
            }
        }

        total_price
    }


    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");



    fn calculate_total_price_with_sides(grid: Vec<Vec<char>>) -> usize {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut visited = vec![vec![false; cols]; rows];
        let mut total_price = 0;

        // Traverse the grid to find regions
        for row in 0..rows {
            for col in 0..cols {
                if !visited[row][col] {
                    let (area, _, sides) = flood_fill(&grid, &mut visited, row, col, &DIRECTIONS);
                    total_price += area * sides;
                }
            }
        }

        total_price
    }





    fn find_sides(region: &HashSet<(isize, isize)>) -> usize {

        let mut sides = 0;

        for &dir in &DIRECTIONS {
            let mut found: HashSet<(isize, isize)> = HashSet::new();
        for cell in region {
            let neighbor = (cell.0 + dir.0, cell.1 + dir.1);
            if !region.contains(&neighbor) && !found.contains(cell) {
                // this is a side, and we need to find adjacent's on the same side
                found.insert(*cell);
                sides += 1;
            } else {
                    continue;
                }

            // if top, go left, then right...
            let left = (dir.1, dir.0);
            let right = (-dir.1, -dir.0);

            for lr_dir in [left, right] {
                let mut cur = *cell;
                loop {
                    cur.0 += lr_dir.0;
                    cur.1 += lr_dir.1;
                    let check = &(cur.0 + dir.0, cur.1 + dir.1);

                    if region.contains(&cur) && !region.contains(check) {
                        // found adjacent
                        found.insert(cur);
                    } else {
                        break;
                    }
                }
            }


            }
        }

        sides
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let grid = parse_map(reader);

        let total_price = calculate_total_price_with_sides(grid);
        println!("Total price for fencing: {}", total_price);
        Ok(total_price)
    }

    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
