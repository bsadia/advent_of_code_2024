use std::collections::{HashMap, VecDeque};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn is_update_in_order(update: &[i32], rules: &HashMap<i32, Vec<i32>>) -> bool {
        let mut positions: HashMap<i32, usize> = HashMap::new();


        for (pos, &page) in update.iter().enumerate() {
            positions.insert(page, pos);
        }

        for (&before, after_list) in rules.iter() {
            if let Some(&before_pos) = positions.get(&before) {
                for &after in after_list {
                    if let Some(&after_pos) = positions.get(&after) {
                        if before_pos > after_pos {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    fn part1<R: BufRead>(reader: R) -> Result<i32> {

        let (rules_map, updates_vec) =  parse_input(reader);


        let mut middle_sum = 0;


        for (_, update) in updates_vec.iter().enumerate() {
            if is_update_in_order(update, &rules_map) {
                let middle = update[update.len() / 2];
                middle_sum += middle;
            }
        }
        println!("middle_sum: {}", middle_sum);

        Ok(middle_sum)
    }


    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    // Function to reorder an update using topological sort
    fn reorder_update(update: &[i32], rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
        let mut graph = HashMap::new();
        let mut indegree = HashMap::new();

        for &page in update {
            graph.entry(page).or_insert_with(Vec::new);
            indegree.entry(page).or_insert(0);
        }
        // Build the graph
        for &page in update {
            if let Some(following_pages) = rules.get(&page) {
                for &following_page in following_pages {
                    if update.contains(&following_page) {
                        graph.entry(page).or_default().push(following_page);
                        *indegree.entry(following_page).or_default() += 1;
                    }
                }
            }
        }

        let mut queue = VecDeque::new();
        for (&node, &degree) in &indegree {
            if degree == 0 {
                queue.push_back(node);
            }
        }

        let mut sorted = Vec::new();
        while let Some(node) = queue.pop_front() {
            sorted.push(node);
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    let count = indegree.get_mut(&neighbor).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        sorted
    }
    fn parse_input<R: BufRead>(reader: R) -> (HashMap<i32, Vec<i32>> ,Vec<Vec<i32>>) {
        let lines: Vec<String> = reader
            .lines()
            .filter_map(Result::ok) // Ignore errors
            .collect();

        let blank_line_index = lines.iter().position(|line| line.trim().is_empty()).unwrap();



        let rules = &lines[..blank_line_index];
        let updates = &lines[blank_line_index + 1..];


        let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for rule in rules {
            let parts: Vec<i32> = rule.split('|').map(|x| x.parse::<i32>().unwrap()).collect();
            rules_map.entry(parts[0]).or_default().push(parts[1]);
        }



        let updates_vec: Vec<Vec<i32>> = updates
            .iter()
            .map(|line| {
                line.split(',')
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>() // Collect into Vec<i32>
            })
            .collect();

        (rules_map, updates_vec)
    }

    fn part2<R: BufRead>(reader: R) -> Result<i32> {

        let (rules_map, updates_vec) =  parse_input(reader);

        let mut middle_sum = 0;

        let mut corrected_updates = Vec::new();

        for update in &updates_vec {
            if ! is_update_in_order(update, &rules_map){
                let corrected = reorder_update(update, &rules_map);
                let middle = corrected[corrected.len() / 2];
                corrected_updates.push(corrected);

                middle_sum += middle;
            }
        }

        println!("Middle_sum: {}", middle_sum);

        Ok(middle_sum)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
