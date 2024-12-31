use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::cmp::{min, Reverse};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct State {
    position: (usize, usize),
    direction: char,
    cost: usize,
    path: HashSet<(usize, usize)>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // We prioritize the state with the lower cost.
        Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the ordering to make BinaryHeap act as a min-heap.
        other.cost.cmp(&self.cost) // Reverse the order to get a min-heap
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    // Define the direction options and their corresponding delta movements
    fn get_options() -> HashMap<char, (Vec<char>, (isize, isize))> {
        let mut options = HashMap::new();
        options.insert('^', (vec!['>', '<'], (-1, 0)));
        options.insert('>', (vec!['v', '^'], (0, 1)));
        options.insert('v', (vec!['<', '>'], (1, 0)));
        options.insert('<', (vec!['^', 'v'], (0, -1)));
        options
    }


    // Depth-first search function to calculate the minimum score
    fn dfs(
        mat: &Vec<Vec<char>>,
        start: (usize, usize),
        end: (usize, usize),
        options: &HashMap<char, (Vec<char>, (isize, isize))>,
    ) -> HashMap<char, usize> {
        let rows = mat.len();
        let cols = mat[0].len();

        // Initialize the values matrix
        let mut values = vec![vec![HashMap::new(); cols]; rows];

        // Queue for BFS
        let mut q = VecDeque::new();
        q.push_back((start.0, start.1, '>', 0));

        while let Some((i, j, d, p)) = q.pop_front() {
            if i >= rows || j >= cols || mat[i][j] == '#' {
                continue;
            }
            if *values[i][j].get(&d).unwrap_or(&usize::MAX) <= p {
                continue;
            }
            values[i][j].insert(d, p);

            if (i, j) == end {
                continue;
            }

            if let Some((opt, delta)) = options.get(&d) {
                // Move in the same direction
                let ni = i.wrapping_add(delta.0 as usize);
                let nj = j.wrapping_add(delta.1 as usize);
                q.push_back((ni, nj, d, p + 1));

                // Rotate to other directions
                for &x in opt {
                    q.push_back((i, j, x, p + 1000));
                }
            }
        }

        values[end.0][end.1].clone()
    }

    fn parse_input<R: BufRead>(reader: R) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {

        let mat: Vec<Vec<char>> = reader.lines()
            .map(|l| l.unwrap().trim().chars().collect())
            .collect();

        let mut start = (0, 0);
        let mut end = (0, 0);

        // Find start (S) and end (E) positions
        for (i, row) in mat.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch == 'S' {
                    start = (i, j);
                }
                if ch == 'E' {
                    end = (i, j);
                }
            }
        }


        (mat, start, end)

    }


    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let (mat, start, end) = parse_input(reader);

        let options = get_options();
        let score = dfs(&mat, start, end, &options);
        // Find the minimum score for reaching the end
        let mut min_score = usize::MAX;
        for &v in score.values() {
            min_score = min(min_score, v);
        }

        println!("Mininum score: {}", min_score);
        Ok(min_score)
    }


    assert_eq!(11048, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn dfs_part2(
        mat: &Vec<Vec<char>>,
        start: (usize, usize),
        end: (usize, usize),
    ) -> HashMap<char, (usize, HashSet<(usize, usize)>)> {
        let rows = mat.len();
        let cols = mat[0].len();

        let options = get_options();

        let mut values: Vec<Vec<HashMap<char, (usize, HashSet<(usize, usize)>)>>> =
            vec![vec![HashMap::new(); cols]; rows];

        let mut queue = BinaryHeap::new();
        let mut initial_path = HashSet::new();
        initial_path.insert(start);

        queue.push(Reverse((0, State {
            position: start,
            direction: '>',
            cost: 0,
            path: initial_path,
        })));

        while let Some(Reverse((_, State { position: (i, j), direction, cost, mut path }))) =
            queue.pop()
        {
            if i >= rows || j >= cols || mat[i][j] == '#' {
                continue;
            }

            path.insert((i, j));

            let entry = values[i][j].entry(direction).or_insert((usize::MAX, HashSet::new()));

            if cost > entry.0 {
                continue;
            } else if cost == entry.0 {
                entry.1.extend(path.clone());
            } else {
                entry.0 = cost;
                entry.1 = path.clone();
            }

            if (i, j) == end {
                continue;
            }

            if let Some(&(ref rotations, delta)) = options.get(&direction) {
                let (ni, nj) = (
                    i.wrapping_add(delta.0 as usize),
                    j.wrapping_add(delta.1 as usize),
                );

                if ni < rows && nj < cols && mat[ni][nj] != '#' {
                    queue.push(Reverse((cost + 1, State {
                        position: (ni, nj),
                        direction,
                        cost: cost + 1,
                        path: path.clone(),
                    })));
                }

                for &new_dir in rotations {
                    queue.push(Reverse((cost + 1000, State {
                        position: (i, j),
                        direction: new_dir,
                        cost: cost + 1000,
                        path: path.clone(),
                    })));
                }
            }
        }

        values[end.0][end.1].clone()
    }
    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let (mat, start, end) = parse_input(reader);

        // Run the DFS algorithm
        let score = dfs_part2(&mat, start, end);

        // Find the minimum cost and corresponding path
        let mut min_score = usize::MAX;
        let mut best_path = HashSet::new();

        for (_, &(cost, ref path)) in &score {
            if cost < min_score || (cost == min_score && path.len() > best_path.len()) {
                min_score = cost;
                best_path = path.clone();
            }
        }

        println!(" Minimum cost: {}", min_score);
        println!("Path length: {}", best_path.len());
        Ok(best_path.len())
    }

    assert_eq!(64, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
