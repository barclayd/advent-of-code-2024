use std::collections::HashMap;
use itertools::Itertools;
use std::fs;

struct Circuit<'a> {
    values: HashMap<&'a str, u8>,
    gates: HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    outputs: Vec<&'a str>,
}

impl<'a> Circuit<'a> {
    fn from_input(init: &'a str, conn: &'a str) -> Self {
        let mut values = HashMap::new();
        for line in init.lines() {
            let (id, v) = line.split_once(": ").expect("should have `: `");
            values.insert(id, v.parse::<u8>().expect("must be a number"));
        }

        let mut gates = HashMap::new();
        let mut outputs = vec![];
        
        for line in conn.lines() {
            let (gate, out) = line.split_once(" -> ").expect("must have ` -> `");
            let (a, op, b) = gate.split(' ').collect_tuple().expect("invalid gate format");
            gates.insert(out, (a, op, b));
            if out.starts_with('z') {
                outputs.push(out);
            }
        }

        outputs.sort_unstable();
        outputs.reverse();
        
        Self { values, gates, outputs }
    }

    fn evaluate(&mut self) -> u64 {
        let outputs = self.outputs.clone();
        let mut result = 0;
        for &output in &outputs {
            let bit = self.dfs(output);
            result = (result << 1) | (bit as u64);
        }
        result
    }

    fn dfs(&mut self, id: &'a str) -> u8 {
        if let Some(&v) = self.values.get(id) {
            return v;
        }

        let (a, op, b) = self.gates[id];
        let va = self.dfs(a);
        let vb = self.dfs(b);

        let res = match op {
            "AND" => va & vb,
            "XOR" => va ^ vb,
            "OR" => va | vb,
            _ => panic!("unsupported op"),
        };
        
        self.values.insert(id, res);
        res
    }
}

struct WireAnalyzer<'a> {
    wire_map: HashMap<&'a str, Vec<(&'a str, &'a str)>>,
    gate_connections: Vec<[&'a str; 4]>,
}

impl<'a> WireAnalyzer<'a> {
    fn from_input(conn: &'a str) -> Self {
        let mut wire_map: HashMap<&str, Vec<(&str, &str)>> = HashMap::default();
        let mut gate_connections = vec![];

        for line in conn.lines() {
            let (gate, out) = line.split_once(" -> ").expect("must have ` -> `");
            let (a, op, b) = gate.split(' ').collect_tuple().expect("invalid gate format");
            gate_connections.push([a, op, b, out]);
        }

        for &[lhs, op, rhs, ret] in &gate_connections {
            wire_map.entry(lhs).or_default().push((op, ret));
            wire_map.entry(rhs).or_default().push((op, ret));
        }

        Self { wire_map, gate_connections }
    }

    fn find_wrong_outputs(&self) -> String {
        let mut wrong_outputs = vec![];

        for &[lhs, op, rhs, ret] in &self.gate_connections {
            if !self.is_valid_gate(lhs, op, rhs, ret) {
                wrong_outputs.push(ret);
            }
        }

        wrong_outputs.sort_unstable();
        wrong_outputs.join(",")
    }

    fn is_valid_gate(&self, lhs: &str, op: &str, rhs: &str, ret: &str) -> bool {
        let chained_ops = self.wire_map.get(ret);
        let chained_ops_contain = |op| 
            chained_ops.is_some_and(|v| v.iter().any(|a| a.0 == op));

        let has_chained_xor = chained_ops_contain("XOR");
        let has_chained_and = chained_ops_contain("AND");
        let has_chained_or = chained_ops_contain("OR");
        let takes_first_input = lhs.ends_with("00") && rhs.ends_with("00");
        let takes_input_bit = (lhs.starts_with('x') && rhs.starts_with('y'))
            || (rhs.starts_with('x') && lhs.starts_with('y'));
        let outputs_bit = ret.starts_with('z');
        let outputs_last_bit = ret == "z45";

        match op {
            "XOR" => !takes_input_bit && outputs_bit 
                || takes_input_bit && has_chained_xor 
                || takes_first_input && outputs_bit,
            "OR" => outputs_last_bit || (has_chained_and && has_chained_xor),
            "AND" => has_chained_or || takes_first_input,
            _ => unreachable!()
        }
    }
}

fn get_value_for_part_1(file_path: &str) -> u64 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let (init, conn) = contents
        .split_once("\n\n")
        .expect("should have an empty line");
    
    let mut circuit = Circuit::from_input(init, conn);
    circuit.evaluate()
}

fn get_value_for_part_2(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let (_, conn) = contents
        .split_once("\n\n")
        .expect("should have an empty line");
    
    let analyzer = WireAnalyzer::from_input(conn);
    analyzer.find_wrong_outputs()
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
