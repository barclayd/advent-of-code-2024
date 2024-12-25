use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn get_value_for_part_1(file_path: &str) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (init, conn) = file_contents
        .split_once("\n\n")
        .expect("should have an empty line");
    let mut w = HashMap::new();
    for line in init.lines() {
        let (id, v) = line.split_once(": ").expect("should have `: `");
        w.insert(id, v.parse::<u8>().expect("must be a number"));
    }
    let mut g = HashMap::new();
    let mut og = HashMap::new();
    let mut outs = vec![];
    for line in conn.lines() {
        let (gate, out) = line.split_once(" -> ").expect("must have ` -> `");
        let (a, op, b) = gate.split(' ').collect_tuple().expect("");
        g.insert((a, op, b), out);
        og.insert(out, (a, op, b));
        if out.starts_with('z') {
            outs.push(out);
        }
    }
    fn dfs<'a>(
        id: &'a str,
        w: &mut HashMap<&'a str, u8>,
        og: &HashMap<&str, (&'a str, &str, &'a str)>,
    ) -> u8 {
        if let Some(v) = w.get(id) {
            return *v;
        }
        let (a, op, b) = og[id];
        let va = dfs(a, w, og);
        let vb = dfs(b, w, og);

        let res = match op {
            "AND" => va & vb,
            "XOR" => va ^ vb,
            "OR" => va | vb,
            _ => panic!("unsupported op"),
        };
        w.insert(id, res);
        res
    }

    outs.sort_unstable();
    outs.reverse();
    let res = outs.iter().map(|&o| dfs(o, &mut w, &og)).collect_vec();
    res.iter().fold(0, |r, &t| r * 2 + t as u64)
}

fn get_value_for_part_2(file_path: &str) -> String {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (init, conn) = file_contents
        .split_once("\n\n")
        .expect("should have an empty line");
    let mut wire_map: HashMap<&str, Vec<(&str, &str)>> = HashMap::default();
    let mut gate_connections = vec![];
    for line in conn.lines() {
        let (gate, out) = line.split_once(" -> ").expect("must have ` -> `");
        let (a, op, b) = gate.split(' ').collect_tuple().expect("");
        gate_connections.push([a, op, b, out]);
    }

    for &[lhs, op, rhs, ret] in gate_connections.iter() {
        wire_map.entry(lhs).or_insert(vec![]).push((op, ret));
        wire_map.entry(rhs).or_insert(vec![]).push((op, ret));
    }

    let mut wrong_outputs = vec![];
    for &[lhs, op, rhs, ret] in gate_connections.iter() {
        let chained_ops = wire_map.get(&ret);
        let chained_ops_contain =
            |op| chained_ops.is_some_and(|v| v.iter().find(|a| a.0 == op).is_some());

        let has_chained_xor = chained_ops_contain("XOR");
        let has_chained_and = chained_ops_contain("AND");
        let has_chained_or = chained_ops_contain("OR");
        let takes_first_input = lhs.ends_with("00") && rhs.ends_with("00");
        let takes_input_bit = (lhs.starts_with('x') && rhs.starts_with('y'))
            || (rhs.starts_with('x') && lhs.starts_with('y'));
        let outputs_bit = ret.starts_with('z');
        let outputs_last_bit = ret == "z45";

        let valid = match op {
            "XOR" => {
                if !takes_input_bit && outputs_bit {
                    true
                } else if takes_input_bit && has_chained_xor {
                    true
                } else if takes_first_input && outputs_bit {
                    true
                } else {
                    false
                }
            }
            "OR" => {
                if outputs_last_bit || (has_chained_and && has_chained_xor) {
                    true
                } else {
                    false
                }
            }
            "AND" => {
                if has_chained_or {
                    true
                } else if takes_first_input {
                    true
                } else {
                    false
                }
            }
            _ => {
                unreachable!()
            }
        };
        if !valid {
            wrong_outputs.push(ret);
        }
    }

    wrong_outputs.sort_unstable();
    wrong_outputs.join(",").to_string()
}

fn main() {
    println!("Part 1 value: {}", get_value_for_part_1("./input.txt"));
    println!("Part 2 value: {}", get_value_for_part_2("./input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::{get_value_for_part_1, get_value_for_part_2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let value = get_value_for_part_1("./test.txt");
        assert_eq!(value, 2024);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value_for_part_1("./input.txt");
        assert_eq!(value, 45923082839246);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value_for_part_2("./input.txt");
        assert_eq!(value, "jgb,rkf,rrs,rvc,vcg,z09,z20,z24");
    }
}
