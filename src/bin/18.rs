use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::collections::{BinaryHeap};
use std::cmp::Ordering;

// Directions for movement
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];


// Struct to reverse ordering for BinaryHeap
#[derive(Clone, Copy, Eq, PartialEq)]
struct Reverse<T>(T);

impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: Ord> PartialOrd for Reverse<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

fn main() -> Result<()> {

    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");


    fn part1<R: BufRead>(reader: R, dest:(usize, usize), limit:usize) -> Result<usize> {


        let points = parse_input(reader);
        let (cost, _) = process_astar(points, dest, limit, true);
        println!("Part 1: {}", cost);



        Ok(0)
    }

    fn process_astar(points: Vec<(usize, usize)>, dest:(usize,usize), limit:usize, part_1: bool)->(usize, (usize, usize)) {

        let mut p = vec![];
        let mut f = false;
        let mut g = 0;

        let mut part_1_cost = 0;
        let mut part_2_points  = (0, 0);

        for i in limit..points.len() {
            let c = points[..=i].to_vec();
            if p.contains(&points[i]) || p.is_empty() {
                let (found, cost, path) = a_star_search((0, 0), dest, &c);
                f = found;
                g = cost;
                p = path;
            }

            if !f {
                println!("Answer {},{}", points[i].1, points[i].0);
                part_2_points = (points[i].1, points[i].0);
                break;
            }

            if part_1 {
                println!("Part 1: cost {}", g);
                part_1_cost = g;
                break;
            }
        }
        (part_1_cost, part_2_points)
    }


    assert_eq!(0, part1(BufReader::new(TEST.as_bytes()), (6,6), 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file,(70,70), 1024)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");



    // Manhattan distance heuristic
    fn heuristic(i: usize, j: usize, ti: usize, tj: usize) -> usize {
        (i as isize - ti as isize).abs() as usize + (j as isize - tj as isize).abs() as usize
    }

    // Trace path from the details matrix


    fn trace_path(details: &Vec<Vec<(usize, usize, usize, usize, usize)>>, dst: (usize, usize)) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        let (mut i, mut j) = dst;

        // Traverse back using the parent information in `details`
        while details[i][j].0 != i || details[i][j].1 != j {
            path.push((i, j));
            let parent = details[i][j];
            i = parent.0;
            j = parent.1;
        }

        path.push((i, j));
        path.reverse();
        path
    }

    // A* Algorithm
    fn a_star_search(
        src: (usize, usize),
        dst: (usize, usize),
        points: &Vec<(usize, usize)>,
    ) -> (bool, usize, Vec<(usize, usize)>) {

        let mut closed = vec![vec![false; dst.0+1]; dst.0+1];

        let mut details = vec![vec![(0, 0, usize::MAX, usize::MAX, 0); dst.0+1]; dst.0+1];


        let (i, j) = src;
        details[i][j] = (i, j, 0, 0, 0);

        let mut list: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::new();
        list.push((Reverse(0), i, j));

        while let Some((Reverse(_), i, j)) = list.pop() {
            closed[i][j] = true;

            for d in DIRECTIONS.iter() {
                let ni = i as isize + d.0;
                let nj = j as isize + d.1;
                // !valid((ni, nj))
                if !(ni >= 0 && ni <= dst.0 as isize && nj >= 0 && nj<= dst.0 as isize)
                {
                    continue;
                }

                let (ni, nj) = (ni as usize, nj as usize);
                if points.contains(&(ni, nj)) || closed[ni][nj] {
                    continue;
                }

                let g = details[i][j].3 + 1;

                if (ni, nj) == dst {
                    details[ni][nj] = (i, j, g, g, 0);
                    return (true, g, trace_path(&details, dst));
                } else {
                    let h = heuristic(i, j, dst.0, dst.1);
                    let f = g + h;

                    if details[ni][nj].2 == usize::MAX || details[ni][nj].2 > f {
                        list.push((Reverse(f), ni, nj));
                        details[ni][nj] = (i, j, f, g, h);
                    }
                }
            }
        }
        (false, 0, vec![])
    }

fn parse_input<R: BufRead>(reader: R) ->Vec<(usize, usize)> {
    let points: Vec<(usize, usize)> = reader
        .lines()
        .filter_map(|l| {
            l.ok().and_then(|line| {
                let parts: Vec<_> = line.split(',').collect();
                if parts.len() == 2 {
                    Some((parts[1].trim().parse().unwrap(), parts[0].trim().parse().unwrap()))
                } else {
                    None
                }
            })
        })
        .collect();
    points
}

    fn part2<R: BufRead>(reader: R, dest:(usize, usize), limit:usize) -> Result<usize> {

        let points = parse_input(reader);

        let (_, point) = process_astar(points, dest, limit, false);
        println!("Part 2: {:?}", point );
        Ok(0)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()), (6,6), 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, (70, 70), 1024)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
