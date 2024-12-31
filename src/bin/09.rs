use std::fs::File;
use anyhow::*;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";
//38243793313545896186614619720332973424914628059168687 for testing only

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_disk_map<R: BufRead>(reader: R) -> Vec<Option<usize>> {
        let mut result = Vec::new(); // To store the parsed representation
        let mut file_id = 0; // Start file ID from 0

        for line in reader.lines() {
            let disk_map = line.unwrap(); // Read each line as a string
            let mut chars = disk_map.chars(); // Iterator over the characters

            while let Some(file_size_char) = chars.next() {
                let file_size = file_size_char.to_digit(10).unwrap() as usize;

                // Push file ID into the result for `file_size` times
                result.extend(std::iter::repeat(Some(file_id)).take(file_size));

                file_id += 1;

                // Parse free space size (if any)
                if let Some(free_space_char) = chars.next() {
                    let free_space = free_space_char.to_digit(10).unwrap() as usize;

                    // Push `None` into the result for `free_space` times
                    result.extend(std::iter::repeat(None).take(free_space));
                }
            }
        }

        result
    }

    fn move_one_file_at_time(disk_map: Vec<Option<usize>>)->usize {
        let mut map_copy = disk_map.clone();
        let mut left = 0; // Pointer to find the first free space

        let mut right = map_copy.len() - 1; // Pointer to find the last file block

        while left < right {
            // Move `left` to the next free space (None)
            while left < map_copy.len() && map_copy[left].is_some() {
                left += 1;
            }

            // Move `right` to the previous file block (Some(file_id))
            while right > 0 && map_copy[right].is_none() {
                right -= 1;
            }

            // If we found both an empty space and a file block, swap them
            if left < right && map_copy[left].is_none() && map_copy[right].is_some() {
                map_copy.swap(left, right);
                left += 1; // Move to the next free space
                right -= 1; // Move to the previous file block
            }
        }

        map_copy
            .iter()
            .enumerate()
            .filter_map(|(position, &file_id)| file_id.map(|id| position * id))
            .sum()


    }


    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let disk_map = parse_disk_map(reader);
        let count = move_one_file_at_time(disk_map);

        println!("checksum: {}", count);
        Ok(count)
    }


    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);

    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn move_file_blocks(disk_map: &mut Vec<Option<usize>>) -> usize {
        let left = disk_map
            .iter()
            .position(|x| x.is_none())
            .unwrap(); // Start pointer for free space
        let mut right = disk_map.len() - 1; // Start pointer for file blocks
        let max_file_id = disk_map.iter().filter_map(|&x| x).max().unwrap_or(0);
        let mut moved = vec![false; max_file_id]; // Track moved file IDs

        while left < right {
            if let Some(file_id) = disk_map[right] {

                let mut need = 0;
                for idx in (0..=right).rev() {
                    if disk_map[idx] == Some(file_id) {
                        need += 1;
                    } else {
                        break;
                    }
                }

                if moved[file_id - 1] {
                    right -= need;
                    continue;
                }

                let mut free_space = left; // Start looking for free space from `left`

                loop {
                    // 1. Find the next free space (skip the occupied spaces)
                    while disk_map[free_space].is_some() {
                        free_space += 1; // Skip occupied space
                    }

                    // 2. Check if there is enough space to move the file (not enough space, move to next file)
                    if free_space > right - need {
                        right -= need; // Move the right pointer to next block (file)
                        break;
                    }

                    // 3. Count how many free spaces are available from the current free_space
                    let mut available = 0;
                    for i in free_space.. {
                        if disk_map[i].is_none() {
                            available += 1;
                        } else {
                            break; // Stop once we hit an occupied space
                        }
                    }

                    // 4. Check if the available space is enough to fit the current file (need)
                    if available >= need {
                        // Move the file to the free space
                        for i in free_space..free_space + need {
                            disk_map[i] = Some(file_id); // Place the file ID in the free space
                        }

                        // Clear the original positions of the file from the `right` end
                        for i in right - need + 1..=right {
                            disk_map[i] = None; // Remove the original file ID
                        }

                        // Update the `right` pointer
                        right -= need;
                        moved[file_id - 1] = true; // Mark the file as moved
                        break;
                    } else {
                        // Not enough space found, move to the next free space
                        free_space += available; // Skip to the next available space
                    }
                }
            } else {
                right -= 1; // Skip if already free space
            }
        }

        // Compute checksum as sum of position * file_id
        disk_map
            .iter()
            .enumerate()
            .filter_map(|(position, &file_id)| file_id.map(|id| position * id))
            .sum()
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut disk_map = parse_disk_map(reader);

        let count = move_file_blocks( &mut disk_map);

        println!("checksum: {}", count);

        Ok(count)
    }


    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
