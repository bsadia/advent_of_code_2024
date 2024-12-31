use std::result::Result::Ok;
use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "24";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn work(values: &mut HashMap<String, u32>, ops: &[(String, String, String, String)]) -> usize {
        let mut missing = 0;

        for (inp1, inp2, gate, out) in ops {
            if values.contains_key(out) {
                continue;
            }

            if !values.contains_key(inp1) || !values.contains_key(inp2) {
                missing += 1;
                continue;
            }

            let result = match gate.as_str() {
                "OR" => values[inp1] | values[inp2],
                "AND" => values[inp1] & values[inp2],
                "XOR" => (values[inp1] as i64 ^ values[inp2] as i64) as u32,
                _ => panic!("undefined gate {}", gate),
            };

            values.insert(out.clone(), result);
        }

        missing
    }

    fn part1<R: BufRead>(reader: R) -> Result<u64> {


        let (mut values, ops) = parse_input(reader);

        while work(&mut values, &ops) > 0 {}

        let mut output = 0;
        for (key, value) in &values {
            if key.starts_with('z') {
                let index: usize = key[1..].parse().expect("Invalid z-index");
                let shifted_value = (*value as u64) << index;
               output |= shifted_value;
            }
        }

        println!("{}", output);
        Ok(output)
    }


    assert_eq!(2024, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    fn parse_input<R: BufRead>(reader: R) -> (HashMap<String, u32>, Vec<(String, String, String, String)>) {

        let mut lines = reader.lines();

        // Parse values
        let mut values: HashMap<String, u32> = HashMap::new();
        let mut ops: Vec<(String, String, String, String)> = Vec::new();

        // Read the first part of the input
        while let Some(Ok(line)) = lines.next() {
            if line.trim().is_empty() {
                break;
            }
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() == 2 {
                values.insert(parts[0].to_string(), parts[1].parse().unwrap());
            }
        }

        // Read the second part of the input
        for line in lines.filter_map(Result::ok) {
            if line.trim().is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split(" -> ").collect();
            if parts.len() == 2 {
                let expr: Vec<&str> = parts[0].split_whitespace().collect();
                if expr.len() == 3 {
                    ops.push((
                        expr[0].to_string(),
                        expr[2].to_string(),
                        expr[1].to_string(),
                        parts[1].to_string(),
                    ));
                }
            }
        }
        (values, ops)

    }

    //region Part 2
    println!("\n=== Part 2 ===");

    /// Function to compute the highest bit of an integer.
    fn highest_bit(n: u64) -> Option<usize> {
        if n == 0 {
            return None;
        }
        let mut bit = 0;
        let mut n = n;
        while n > 0 {
            n >>= 1;
            bit += 1;
        }
        Some(bit - 1)
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let (_, ops) = parse_input(reader);

        // Calculate highest bit
        let output = time_snippet!(part1(BufReader::new(File::open(INPUT_FILE)?)));
        let last_output = format!("z{:02}", highest_bit(output?).unwrap());

        println!("highest bit is {}", last_output);

        // Track usage and errors
        let mut usage: HashMap<String, HashSet<String>> = HashMap::new();
        let mut errors: Vec<String> = Vec::new();

        for (inp1, inp2, gate, _) in &ops {
            usage.entry(inp1.clone()).or_default().insert(gate.clone());
            usage.entry(inp2.clone()).or_default().insert(gate.clone());
        }

        for (inp1, inp2, gate, out) in &ops {
            if out == &last_output {
                if inp1.starts_with('x') || inp1.starts_with('y') || inp2.starts_with('x') || inp2.starts_with('y') || gate != "OR" {
                    errors.push(out.clone());
                }
                continue;
            }

            if out == "z00" {
                if [&inp1[..], &inp2[..]] != ["x00", "y00"] || gate != "XOR" {
                    errors.push(out.clone());
                }
                continue;
            }

            if inp1 == "x00" || inp1 == "y00" || inp2 == "x00" || inp2 == "y00" {
                if (inp1.starts_with('x') && inp2.starts_with('y')) || (inp1.starts_with('y') && inp2.starts_with('x')) {
                    if gate != "XOR" && gate != "AND" {
                        errors.push(out.clone());
                    }
                }
                continue;
            }

            match gate.as_str() {
                "XOR" => {
                    if inp1.starts_with('x') || inp1.starts_with('y') {
                        if !inp2.starts_with('x') && !inp2.starts_with('y') {
                            errors.push(out.clone());
                        }
                        if out.starts_with('z') {
                            errors.push(out.clone());
                        }
                        if !usage.get(out).map_or(false, |g| g.contains("AND") && g.contains("XOR")) {
                            errors.push(out.clone());
                        }
                    } else if !out.starts_with('z') {
                        errors.push(out.clone());
                    }
                }
                "OR" => {
                    if inp1.starts_with('x') || inp1.starts_with('y') || inp2.starts_with('x') || inp2.starts_with('y') || out.starts_with('z') {
                        errors.push(out.clone());
                    }
                    if !usage.get(out).map_or(false, |g| g.contains("AND") && g.contains("XOR")) {
                        errors.push(out.clone());
                    }
                }
                "AND" => {
                    if inp1.starts_with('x') || inp1.starts_with('y') {
                        if !inp2.starts_with('x') && !inp2.starts_with('y') {
                            errors.push(out.clone());
                        }
                    }
                    if !usage.get(out).map_or(false, |g| g.contains("OR")) {
                        errors.push(out.clone());
                    }
                }
                _ => {}
            }
        }

        errors.sort();
        errors.dedup();
        println!("Length: {}", errors.len());
        println!("{}", errors.join(","));

        Ok(0)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
