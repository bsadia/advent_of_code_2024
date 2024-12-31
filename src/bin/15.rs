use std::result::Result::Ok;
use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;


const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

// ##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########
//
// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
const TEST: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";


fn main() -> Result<()> {
    start_day(DAY);

    fn find_start_position(map: &[Vec<char>]) -> (usize, usize) {


        for (x, row) in map.iter().enumerate() {
            for (y, &ch) in row.iter().enumerate() {
                if ch == '@' {
                    return (x, y);
                }
            }
        }
        (usize::MAX, usize::MAX) // Should not reach here
    }

    //region Part 1
    println!("=== Part 1 ===");



    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let (map, moves)=  read_input_combined(reader, false);


        let mut robot_pos = find_start_position(&map);

        let mut boxes = HashSet::new();


        // Parse the map
        for (i, row) in map.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                if  *ch == 'O' {
                    boxes.insert((i, j)); // Add box positions to the set
                }
            }
        }

        // Process moves
        for &dir in &moves {
            let (dx, dy) = match dir {
                '^' => (-1, 0),
                'v' => (1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                _ => continue,
            };

            let new_robot_pos = (
                (robot_pos.0 as isize + dx) as usize,
                (robot_pos.1 as isize + dy) as usize,
            );
           // println!("{:?}", robot_pos);

            if map[new_robot_pos.0][new_robot_pos.1] == '#' {
                continue; // Robot hits a wall, no movement
            }

            let mut current_pos = new_robot_pos;
            let mut box_chain = Vec::new();

            // Detect and collect the chain of boxes in the move direction
            while boxes.contains(&current_pos) {
                box_chain.push(current_pos);
                current_pos = (
                    (current_pos.0 as isize + dx) as usize,
                    (current_pos.1 as isize + dy) as usize,
                );
            }

            // Check if the chain of boxes can move
            if map[current_pos.0][current_pos.1] == '#' || boxes.contains(&current_pos) {
                continue; // Blocked: neither the robot nor the boxes can move
            }

            // Move the chain of boxes
            for &pos in box_chain.iter().rev() {
                boxes.remove(&pos);
                let new_pos = (
                    (pos.0 as isize + dx) as usize,
                    (pos.1 as isize + dy) as usize,
                );
                boxes.insert(new_pos);
            }

            // Move the robot
            robot_pos = new_robot_pos;
        }

        // Calculate GPS sum
        let gps_sum: usize = boxes
            .iter()
            .map(|&(r, c)| 100 * r + c)
            .sum();

        println!("Sum of GPS coordinates: {}", gps_sum);
        Ok(gps_sum)
    }

    assert_eq!(908, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion


    fn read_input_combined<R: BufRead>(reader: R, part_2:bool) -> (Vec<Vec<char>>, Vec<char>) {
        let mut map = Vec::new();
        let mut moves = Vec::new();
        let mut in_map = true;

        // Character mapping for expansion
        let mapping = HashMap::from([
            ('#', "##"),
            ('O', "[]"),
            ('.', ".."),
            ('@', "@."),
        ]);

        let mut moves_buffer = String::new(); // Buffer for moves string

        for line in reader.lines().flatten() {
            if line.trim().is_empty() {
                in_map = false; // Blank line marks transition to moves
                continue;
            }
            if in_map {
                let mut row = Vec::new();
                for c in line.chars() {
                    if part_2 {
                        if let Some(mapped) = mapping.get(&c) {
                            row.extend(mapped.chars());
                        }
                    } else {
                        row.push(c);
                    }

                }
                map.push(row);
            } else {
                moves_buffer.push_str(&line.trim()); // Append moves after the blank line
            }
        }

        moves.extend(moves_buffer.chars()); // Convert moves string into Vec<char>
        (map, moves)
    }

    //region Part 2
    println!("\n=== Part 2 ===");


    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let (mut map, moves) = read_input_combined(reader, true);
        let mut position = find_start_position(&map);

        for &dir in &moves {

            let (dx, dy) = match dir {
                '^' => (-1, 0),
                'v' => (1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                _ => continue,
            };

            // Calculate the next position of the robot.
            let ( nx, ny) = (
                (position.0 as isize + dx) as usize,
                (position.1 as isize + dy) as usize,
            );

            // Case 1: Move the robot to an empty space.
            if map[nx][ny] == '.' {
                map[position.0][position.1] = '.'; // Clear the robot's previous position.
                map[nx][ny] = '@';               // Move the robot to the new position.
                position = (nx, ny);
                continue;
            }

            // Case 2: Handle movement when encountering boxes.
            if map[nx][ny] == '[' || map[nx][ny] == ']' {
                let mut boxes = vec![(nx, ny)];

                // Add the other side of the box to the list.
                if map[nx][ny] == '[' {
                    boxes.push((nx, ny + 1));
                } else {
                    boxes.push((nx, ny - 1));
                }

                let mut blocked = false;

                // Handle vertical movement with complex box arrangements.
                if dir == '^' || dir == 'v' {
                    blocked = handle_vertical_boxes(&mut boxes, dx, dy, &map);
                }
                // Handle horizontal movement with straight-line box arrangements.
                else if dir == '<' || dir == '>' {
                    blocked = handle_horizontal_boxes(&mut boxes, dx, dy, &map);
                }

                // If not blocked, move all boxes and the robot.
                if !blocked {
                    move_boxes_and_robot(&mut map, &mut boxes, dx, dy);
                    map[position.0][position.1] = '.'; // Clear the robot's previous position.
                    map[nx][ny] = '@';                // Move the robot to the new position.
                    position = (nx, ny);
                }
            }
        }

        let mut result = 0;
        for (x, row) in map.iter().enumerate() {
            for (y, &cell) in row.iter().enumerate() {
                if cell == '[' {
                    result += 100 * x + y;
                }
            }
        }

        println!("PART 2: {}", result);
        Ok(result)
    }


    /// Handles vertical movement with boxes and checks for blocked paths.
    fn handle_vertical_boxes(
        boxes: &mut Vec<(usize, usize)>,
        dx: isize,
        dy: isize,
        map: &Vec<Vec<char>>,
    ) -> bool {
        let mut current = boxes.clone();

        while current.len() > 1 {
            let mut next = Vec::new();

            for &(x, y) in &current {
                let (pathx, pathy) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

                if map[pathx][pathy] == '#' {
                    return true; // Blocked by a wall.
                }

                if map[pathx][pathy] == '[' || map[pathx][pathy] == ']' {
                    if !next.contains(&(pathx, pathy)) {
                        boxes.push((pathx, pathy));
                        next.push((pathx, pathy));

                        if map[pathx][pathy] == '[' {
                            boxes.push((pathx, pathy + 1));
                            next.push((pathx, pathy + 1));
                        } else {
                            boxes.push((pathx, pathy - 1));
                            next.push((pathx, pathy - 1));
                        }
                    }
                }
            }

            current = next;
        }

        false
    }

    /// Handles horizontal movement with boxes and checks for blocked paths.
    fn handle_horizontal_boxes(
        boxes: &mut Vec<(usize, usize)>,
        dx: isize,
        dy: isize,
        map: &Vec<Vec<char>>,
    ) -> bool {
        let (mut pathx, mut pathy) = (
            (boxes[0].0 as isize + dx) as usize,
            (boxes[0].1 as isize + dy) as usize,
        );

        while map[pathx][pathy] == '[' || map[pathx][pathy] == ']' {
            if !boxes.contains(&(pathx, pathy)) {
                boxes.push((pathx, pathy));
            }

            pathx = (pathx as isize + dx) as usize;
            pathy = (pathy as isize + dy) as usize;
        }

        // Check if the path is blocked by something other than an empty space.
        map[pathx][pathy] != '.'
    }

    /// Moves all boxes in the list and updates the map.
    fn move_boxes_and_robot(map: &mut Vec<Vec<char>>, boxes: &mut Vec<(usize, usize)>, dx: isize, dy: isize) {
        for &(x, y) in boxes.iter().rev() {
            let (movx, movy) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            map[movx][movy] = map[x][y]; // Move the box to its new position.
            map[x][y] = '.';            // Clear the box's old position.
        }
    }

    assert_eq!(618, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
