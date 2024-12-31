use std::collections::{HashMap, VecDeque};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

use itertools::Itertools;
use rayon::prelude::*;
use std::sync::Mutex;


const DAY: &str = "21";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
029A
980A
179A
456A
379A
";

fn main() -> Result<()> {
    start_day(DAY);

    fn get_dir() -> HashMap<char, (isize, isize)> {
        let mut directions = HashMap::new();
        directions.insert('^', (-1, 0));
        directions.insert('v', (1, 0));
        directions.insert('<', (0, -1));
        directions.insert('>', (0, 1));
        directions
    }

    // Tables for coordinates
    fn number_table() -> HashMap<char, (isize, isize)> {
        [
            ('7', (0, 0)),
            ('8', (0, 1)),
            ('9', (0, 2)),
            ('4', (1, 0)),
            ('5', (1, 1)),
            ('6', (1, 2)),
            ('1', (2, 0)),
            ('2', (2, 1)),
            ('3', (2, 2)),
            ('0', (3, 1)),
            ('A', (3, 2)),
        ]
            .iter()
            .cloned()
            .collect()
    }

    fn small_table() -> HashMap<char, (isize, isize)> {
        [
            ('^', (0, 1)),
            ('A', (0, 2)),
            ('<', (1, 0)),
            ('v', (1, 1)),
            ('>', (1, 2)),
        ]
            .iter()
            .cloned()
            .collect()
    }

    struct Solver {
        max_depth: usize,
    }

    impl Solver {
        fn new(max_depth: usize) -> Self {
            Self { max_depth }
        }

        fn simulate(
            &self,
            line: &str,
            mut py: isize,
            mut px: isize,
            gapy: isize,
            gapx: isize,
            directions: &HashMap<char, (isize, isize)>,
        ) -> bool {
            for c in line.chars() {
                if c == 'A' {
                    continue;
                }
                let (dy, dx) = directions.get(&c).unwrap();
                py += dy;
                px += dx;
                if py == gapy && px == gapx {
                    return false;
                }
            }
            true
        }

        fn small_step(
            &self,
            py: isize,
            px: isize,
            ny: isize,
            nx: isize,
            depth: isize,
            directions: &HashMap<char, (isize, isize)>,
            memo: &mut HashMap<(isize, isize, isize, isize, isize), usize>,
            best_so_far: usize,
        ) -> usize {
            if let Some(&cached) = memo.get(&(py, px, ny, nx, depth)) {
                return cached;
            }

            let dy = ny - py;
            let dx = nx - px;
            let mut best = usize::MAX;

            for perm in (0..4).permutations(4) {
                let mut ans = String::new();
                for &p in &perm {
                    match p {
                        0 if dx > 0 => ans += &">".repeat(dx.abs() as usize),
                        1 if dy > 0 => ans += &"v".repeat(dy.abs() as usize),
                        2 if dx < 0 => ans += &"<".repeat(dx.abs() as usize),
                        3 if dy < 0 => ans += &"^".repeat(dy.abs() as usize),
                        _ => {}
                    }
                }
                ans.push('A');
                let (gapy, gapx) = if depth >= 0 { (0, 0) } else { (3, 0) };
                if self.simulate(&ans, py, px, gapy, gapx, directions) {
                    let candidate =
                        self.walk_line(&ans, depth + 1, directions, memo, best_so_far.min(best));
                    if candidate < best_so_far {
                        best = best.min(candidate);
                    }
                }
            }

            memo.insert((py, px, ny, nx, depth), best);
            best
        }

        fn walk_line(
            &self,
            line: &str,
            depth: isize,
            directions: &HashMap<char, (isize, isize)>,
            memo: &mut HashMap<(isize, isize, isize, isize, isize), usize>,
            best_so_far: usize,
        ) -> usize {
            if depth == self.max_depth as isize {
                return line.len();
            }

            let table = if depth >= 0 {
                small_table()
            } else {
                number_table()
            };

            let mut py = table[&'A'].0;
            let mut px = table[&'A'].1;
            let mut size = 0;

            for c in line.chars() {
                let (ny, nx) = table[&c];
                size += self.small_step(py, px, ny, nx, depth, directions, memo, best_so_far);
                py = ny;
                px = nx;
            }

            size
        }
    }


    fn solve(data: Vec<String>, max_depth: usize) -> usize {
        let directions = get_dir();
        let solver = Solver::new(max_depth);
        let memo = Mutex::new(HashMap::new()); // Wrap memo in a Mutex

        data.into_par_iter()
            .map(|line| {
                let n: usize = line[..3].parse().unwrap();
                let steps = {
                    let mut memo_guard = memo.lock().unwrap(); // Lock the Mutex to get mutable access
                    solver.walk_line(&line, -1, &directions, &mut *memo_guard, usize::MAX)
                };
                n * steps
            })
            .sum()
    }
    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let data: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let output = solve(data, 2);

        println!("P1 answer: {:?}", output);

        Ok(output)
    }


    assert_eq!(126384, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let data: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let output = solve(data, 25);

        println!("P2 answer: {:?}", output);

        Ok(0)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
