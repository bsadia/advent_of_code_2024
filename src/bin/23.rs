use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;


const DAY: &str = "23";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn find_or_create_node(nodes: &mut HashMap<String, HashSet<String>>, name: &str) {
        nodes.entry(name.to_string()).or_insert_with(HashSet::new);
    }

    fn parse_input<R: BufRead>(reader: R) -> HashMap<String, HashSet<String>> {

        let raw_data: Vec<String> = reader.lines().filter_map(Result::ok).collect();
        let raw_data = raw_data.join("\n");

        let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();

        // Parse input and build nodes with connections
        for line in raw_data.lines() {
            let mut parts = line.split('-');
            let name1 = parts.next().unwrap();
            let name2 = parts.next().unwrap();

            // Ensure both nodes exist in the map
            find_or_create_node(&mut nodes, name1);
            find_or_create_node(&mut nodes, name2);

            // Add the connection between the nodes
            nodes.get_mut(name1).unwrap().insert(name2.to_string());
            nodes.get_mut(name2).unwrap().insert(name1.to_string());
        }
        nodes

    }

    fn computer_starts_with_t(nodes:  HashMap<String, HashSet<String>>)-> usize {

        let mut connections = HashSet::new();

        // Find valid connections
        for (node, connections_set) in &nodes {
            for c1 in connections_set {
                for c2 in &nodes[c1] {
                    if node == c1 || c1 == c2 || !connections_set.contains(c2) {
                        continue;
                    }

                    let mut connection = vec![node.clone(), c1.clone(), c2.clone()];
                    connection.sort();
                    connections.insert(connection);
                }
            }
        }

        // Count connections where the name starts with "t"
            connections
            .iter()
            .filter(|connection| connection.iter().any(|name| name.starts_with('t')))
            .count()

    }


    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let nodes = parse_input(reader);
        let count = computer_starts_with_t(nodes);

        println!("{}", count);
        Ok(count)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn find_password(nodes: &HashMap<String, HashSet<String>>) -> Vec<String> {
        let mut connections: Vec<Vec<String>> = Vec::new();


        for (node_value, connections_set) in nodes {
            let mut connection = vec![node_value.clone()];

            for c1 in connections_set {
                for c2 in connections_set {
                    if c1 == c2 {
                        continue;
                    }

                    // Check if c1 is in c2's connections
                    if let Some(c2_connections) = nodes.get(c2) {
                        if c2_connections.contains(c1) {
                            connection.push(c1.clone());
                            connection.push(c2.clone());
                        }
                    }
                }
            }

            if connection.len() <= 1 {
                continue;
            }

            // Sort and deduplicate connection
            connection.sort();
            connection.dedup();
            connections.push(connection);
        }

        // Find interconnections
        let mut intercons: Vec<Vec<String>> = Vec::new();
        for c in &connections {
            let count = connections.iter().filter(|x| x == &c).count();
            if c.len() == count {
                intercons.push(c.clone());
            }
        }

        // Find password (max length interconnection)
        intercons.iter()
            .max_by_key(|x| x.len())
            .unwrap_or(&Vec::new())
            .clone()
    }



    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let nodes = parse_input(reader);
        let password = find_password(&nodes);
        println!("password:  {}", password.join(","));

        Ok(0)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
