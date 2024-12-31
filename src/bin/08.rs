use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";



fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_grid<R: BufRead>(reader: R) -> HashMap<(isize, isize), char> {
        let mut grid = HashMap::new();
        for (row, line) in reader.lines().flatten().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                    grid.insert((row as isize, col as isize), ch);
            }
        }
        grid
    }


    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let antinode_length = find_antinode_locations(reader, true );

        Ok(antinode_length)
    }


    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

   // Subtract points (vector subtraction)
    fn subtract_points(a: (isize, isize), b: (isize, isize)) -> Option<(isize, isize)> {
       Some ((a.0 - b.0, a.1 - b.1))
    }

    // Add points (vector addition)
    fn add_points(a: (isize, isize), b: (isize, isize)) -> Option<(isize, isize)> {
        Some((a.0 + b.0, a.1 + b.1))
    }


    fn find_antinode_locations<R: BufRead>(reader: R, part_1:bool)->usize{
        let grid = parse_grid(reader);


        let frequencies: HashSet<char> = grid.values().copied().filter(|&v| v != '.').collect();

        let mut antinode_locations = HashSet::new();


        for &frequency in &frequencies {

            let locations: Vec<_> = grid
                .iter()
                .filter_map(|(&k, &v)| if v == frequency { Some(k) } else { None })
                .collect();

            for i in 0..locations.len() {
                for j in (i + 1)..locations.len() {
                    let l = locations[i];
                    let r = locations[j];
                    // Calculate the slope
                    let slope = subtract_points(l, r).unwrap();

                        if part_1 {
                            for &p in &[add_points(l, slope), subtract_points(r, slope)] {
                                let tuple = p.unwrap();
                                if grid.contains_key(&tuple) {
                                    antinode_locations.insert(tuple);
                                }
                            }
                        }
                        else{
                            for &(mut p, fn_ptr) in &[
                                (l, add_points as fn((isize, isize), (isize, isize)) -> Option<(isize, isize)>),
                                (r, subtract_points as fn((isize, isize), (isize, isize)) -> Option<(isize, isize)>)
                            ] {
                        antinode_locations.insert(p);

                            while let Some(next_p) = fn_ptr(p, slope) {
                                if grid.contains_key(&next_p) {
                                    antinode_locations.insert(next_p);
                                    p = next_p;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("Antinode locations: {:?}", antinode_locations.len());
        antinode_locations.len()
    }
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
       let antinode_length = find_antinode_locations(reader, false );

        Ok(antinode_length)


    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
