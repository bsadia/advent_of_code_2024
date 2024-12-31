use std::collections::HashSet;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

fn main() -> Result<()> {
    start_day(DAY);
    fn parse(line: &str) -> ((isize, isize), (isize, isize)) {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let position = parts[0].split('=').nth(1).unwrap();
        let velocity = parts[1].split('=').nth(1).unwrap();

        let position_coords: Vec<isize> = position
            .split(',')
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        let velocity_coords: Vec<isize> = velocity
            .split(',')
            .map(|x| x.parse::<isize>().unwrap())
            .collect();

        (
            (position_coords[0], position_coords[1]),
            (velocity_coords[0], velocity_coords[1]),
        )
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_input<R: BufRead>(reader: R) -> Vec<((isize, isize), (isize, isize))>{
       reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| parse(&line))
            .collect()
    }

    fn part1<R: BufRead>(reader: R,  size: (isize, isize)) -> Result<usize> {

        // Parse input into a vector of position-velocity pairs
       let robots = parse_input(reader);

        let rounds = 100;
        let (mx, my) = (size.0 / 2, size.1 / 2);

        // Initialize quadrant counts as an array
        let mut quadrant_counts = [0; 4];

        // Process each robot and count its quadrant
        for (position, velocity) in robots {
            let nx = ((position.0 + velocity.0 * rounds) % size.0 + size.0) % size.0; // Wrap around for negative positions
            let ny = ((position.1 + velocity.1 * rounds) % size.1 + size.1) % size.1; // Wrap around for negative positions

            if nx == mx || ny == my {
                continue; // Skip points on the quadrant boundaries
            }
            // Determine the quadrant

            let quadrant = match (nx < mx, ny < my) {
                (true, true) => 0,  // Top-left
                (true, false) => 1, // Top-right
                (false, true) => 2, // Bottom-left
                (false, false) => 3, // Bottom-right
            };
           // println!("{:?} {:?} {:?}", nx, ny, quadrant);
            quadrant_counts[quadrant] += 1;
        }

        // Calculate the product of counts in all quadrants
        let result = quadrant_counts.iter().product::<usize>();

        println!(
            "Top-left: {}, Top-right: {}, Bottom-left: {}, Bottom-right: {}",
            quadrant_counts[0], quadrant_counts[1], quadrant_counts[2], quadrant_counts[3]
        );

        println!("Result: {}", result);

        Ok(result)
    }


    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), (11, 7))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, (101, 103))?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    // Did manually first by printing and inspecting the map  but then consulted reddit and follow the approaches suggested by others.
    fn part2<R: BufRead>(reader: R) -> Result<isize> {
        //Parse the input
        let robots = parse_input(reader);

        let grid_width:isize = 101;
        let grid_height:isize = 103;
        let mut time  = 0;

        for t in 0..grid_width * grid_height {
            // Create a 2D grid to track positions and neighbors
            let mut grid = vec![vec![0; grid_height as usize]; grid_width as usize];
            let mut matching = HashSet::new();

            for &(position, velocity) in &robots {
                // Calculate the next position
                let nx = ((position.0 + t * velocity.0) % grid_width + grid_width) % grid_width;
                let ny = ((position.1 + t * velocity.1) % grid_height + grid_height) % grid_height;


                // If position already has a robot, add to matching set
                if grid[nx as usize][ny as usize] > 0 {
                    matching.insert((nx, ny));
                }

                // Mark the position and its neighbors
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let neighbor_x = (nx + dx).rem_euclid(grid_width) as usize;
                        let neighbor_y = (ny + dy).rem_euclid(grid_height) as usize;
                        grid[neighbor_x][neighbor_y] += 1;
                    }
                }
            }

            // Display grid if matching robots exceed threshold
            if matching.len() > 190 {
                println!("matching = {:?}", matching.len());
                for y in 0..grid_height as usize {
                    for x in 0..grid_width as usize {
                        if grid[x][y] > 0 {
                            print!("*");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                println!("t: {}", t);
                time = t;
                break;
            }
        }


        Ok(time)
    }

    //assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
