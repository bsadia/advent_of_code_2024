use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_map<R: BufRead>(reader: R) -> (Vec<Vec<char>>,
                                            (usize, usize),
                                            char,
                                            HashMap<char, (char, (isize, isize))>) {
        let map: Vec<String> = reader
            .lines()
            .flatten()
            .map(|line| line.trim().to_string())
            .collect();


        let mut options = HashMap::new();
        options.insert('^', ('>', (-1, 0)));
        options.insert('>', ('v', (0, 1)));
        options.insert('v', ('<', (1, 0)));
        options.insert('<', ('^', (0, -1)));

        let mut pos = (0, 0);
        let mut dir = '>';
        let mut map: Vec<Vec<char>> = map.into_iter().map(|line| line.chars().collect()).collect();


        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if options.contains_key(&map[i][j]) {
                    pos = (i, j);
                    dir = map[i][j];
                    break;
                }
            }
        }
        let init_pos = pos;
        let init_dir = dir;


        while (0..map.len()).contains(&pos.0) && (0..map[pos.0].len()).contains(&pos.1) {
            map[pos.0][pos.1] = 'X';


            let (next_dir, delta) = options[&dir];
            let ni = pos.0 as isize + delta.0 ;
            let nj = pos.1 as isize + delta.1 ;

            if ni < 0 || ni >= map.len() as isize || nj < 0 || nj >= map[ni as usize].len() as isize {
                pos = (ni as usize, nj as usize);
                continue;
            }

            if map[ni as usize][nj as usize] == '#' {
                dir = next_dir;
                continue;
            }

            pos = (ni as usize, nj as usize);
        }
        (map, init_pos, init_dir, options)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (map, _, _, _) = parse_map(reader);

        let step = map.iter().flatten().filter(|&&ch| ch == 'X').count();
        println!("{}", step);

        Ok(step)

    }


    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion




    //region Part 2
    println!("\n=== Part 2 ===");

    fn check_bounds(map: &[Vec<char>], x: isize, y: isize ) -> bool {
        x < 0 || x >= map.len() as isize || y< 0 || y >= map[x as usize].len() as isize
    }

    fn get_loop(
        map: &[Vec<char>],
        mut pos: (usize, usize),
        mut dir: char,
        options: &HashMap<char, (char, (isize, isize))>,
        ob: (usize, usize),
    ) -> bool {
        let mut visited = HashSet::new();

        while (0..map.len()).contains(&pos.0) && (0..map[pos.0].len()).contains(&pos.1) {
            if !visited.insert((pos.0, pos.1, dir)) {
                return true;
            }

            let (next_dir, delta) = options[&dir];
            let ni = pos.0 as isize + delta.0;
            let nj = pos.1 as isize + delta.1;

            if check_bounds(map, ni, nj) {
                pos = (ni as usize, nj as usize);
                continue;
            }

            if map[ni as usize][nj as usize] == '#' || (ni as usize, nj as usize) == ob {
                dir = next_dir;
                continue;
            }

            pos = (ni as usize, nj as usize);
        }
        false
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let (map, init_pos, init_dir, options) = parse_map(reader);

        let mut obstacles = Vec::new();
        for (i, row) in map.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch == 'X' {
                    obstacles.push((i, j));
                }
            }
        }

        let mut count = 0;
        for &ob in &obstacles {
            if get_loop(&map, init_pos, init_dir, &options, ob) {
                count += 1;
            }
        }
        println!("Count of loop-causing obstacles: {}", count);
        Ok(count)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
