use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;
use crate::WireState::{Off, On, Unknown};

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

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut wiring = read_wiring(reader)?;
        wiring.update_outputs();

        Ok(wiring.get_wires_value("z"))
    }

    assert_eq!(2024, part1(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut wiring = read_wiring(reader)?;

        dbg!(wiring.get_xy_gates());
        
        Ok(0)
    }
    //
    //assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WireState {
    On,
    Off,
    Unknown,
}

#[derive(Debug, Clone)]
struct WireData {
    state: WireState,
    inputs: Vec<GateId>,
    output: Option<GateId>,
}

impl WireData {
    fn new(state: WireState) -> WireData {
        Self {
            state,
            inputs: vec![],
            output: None,
        }
    }
}

type GateId = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum GateType {
    And,
    Or,
    Xor
}

#[derive(Debug, Clone)]
struct Gate {
    gate_type: GateType,
    inputs: Vec<String>,
    output: String,
}

impl Gate {
    fn new(gate_type: GateType, inputs: Vec<String>, output: String) -> Gate {
        Self { gate_type, inputs, output }
    }
}

#[derive(Debug)]
struct Wiring {
    wires: HashMap<String, WireData>,
    gates: HashMap<GateId, Gate>, // Gate, Input1, Input2, Output
}

impl Wiring {
    fn new(wires: HashMap<String, WireData>, gates: HashMap<GateId, Gate>) -> Wiring {
        Self { wires, gates }
    }

    fn get_xy_gates(&self) -> Result<(Vec<Gate>, Vec<Gate>)>  {
        let x_wires = self.get_wires("x")
            .iter()
            .rev()
            .map(|(wire, _)| wire)
            .cloned()
            .collect::<Vec<_>>();

        let y_wires = self.get_wires("y")
            .iter()
            .rev()
            .map(|(wire, _)| wire)
            .cloned()
            .collect::<Vec<_>>();

        if x_wires.len() != y_wires.len() {
            return Err(anyhow!("wrong number of wires for input"));
        }

        let pairs = x_wires.into_iter().zip(y_wires).collect_vec();
        let mut gates = vec![];

        for (x, y) in pairs {
            for gate in self.gates.values() {
                let inputs = gate.inputs.clone();
                if inputs.contains(&x) && inputs.contains(&y) {
                    gates.push(gate.clone());
                    break;
                }
            }
        }

        let (xor_gates, and_gates): (Vec<Gate>, Vec<Gate>) = gates
            .iter()
            .cloned()
            .partition(|g| g.gate_type == GateType::Xor);


        Ok((xor_gates, and_gates))
    }

    fn get_wires_value(&self, prefix: &str) -> usize {
        let mut ret = 0;
        let wire_states = self.get_wires(prefix);

        for (_, wire_state) in wire_states {
            match wire_state {
                On => ret = 2*ret + 1,
                Off => ret *= 2,
                Unknown => unreachable!(),
            }
        }

        ret
    }

    fn get_wires(&self, prefix: &str) -> Vec<(String, WireState)> {
        let mut ret = vec![];
        for (wire, wire_data) in &self.wires {
            if wire.starts_with(prefix) {
                ret.push((wire.clone(), wire_data.state));
            }
        }
        ret.sort_by(|a, b| b.0.cmp(&a.0));

        ret
    }

    fn update_outputs(&mut self) {
        use WireState::*;
        let mut todos = self.gates.keys().cloned().collect::<Vec<_>>();
        while !todos.is_empty() {
            let mut next_todos = HashSet::new();
            for gate_id in &todos {
                let out_state = self.get_output_state(*gate_id);
                if out_state != Unknown {
                    continue;
                }
                let (in1_state, in2_state) = self.get_inputs(*gate_id);
                match (in1_state, in2_state) {
                    (Unknown, _) | (_, Unknown) => {}
                    (state1, state2) => {
                        let new_out_state = self.calc_output_state(
                            *gate_id,
                            state1,
                            state2);
                        let out = self.get_output(*gate_id);
                        self.update_state(&out, new_out_state);
                        for in_gate in self.get_input_gates(&out) {
                            next_todos.insert(in_gate);
                        }
                    }
                }
            }
            todos = next_todos.iter().cloned().collect::<Vec<_>>();
        }
    }

    fn get_input_gates(&self, wire: &str) -> Vec<GateId> {
        self.wires.get(wire).unwrap().inputs.clone()
    }

    fn update_state(&mut self, wire: &str, state: WireState) {
        self.wires
            .entry(wire.to_string())
            .and_modify(|wire| wire.state = state);
    }

    fn calc_output_state(&self, gate_id: GateId, in1_state: WireState, in2_state: WireState) -> WireState {
        use GateType::*;
        let gate = &self.gates[&gate_id];

        match gate.gate_type {
            And => match (&in1_state, &in2_state) {
                (&On, &On) => On,
                _ => Off,
            }
            Or => match (&in1_state, &in2_state) {
                (&Off, &Off) => Off,
                _ => On,
            }
            Xor => match (&in1_state, &in2_state) {
                (&On, &Off) => On,
                (&Off, &On) => On,
                _ => Off,
            }
        }
    }

    fn get_inputs(&self, gate_id: GateId) -> (WireState, WireState) {
        let gate = &self.gates[&gate_id];
        let in1 = &gate.inputs[0];
        let state1 = &self.wires[in1].state;
        let in2 = &gate.inputs[1];
        let state2 = &self.wires[in2].state;

        (*state1, *state2)
    }

    fn get_output(&self, gate_id: GateId) -> String {
        self.gates[&gate_id].output.clone()
    }

    fn get_output_state(&self, gate_id: GateId) -> WireState {
        let gate = &self.gates[&gate_id];

        self.wires[&gate.output].state
    }
}

fn read_wiring(reader: impl BufRead) -> Result<Wiring> {
    let mut wires = HashMap::new();
    let mut gates = HashMap::new();
    let mut first_section = true;
    let mut next_gate_id = 1;

    for line in read_lines(reader) {
        if line.is_empty() {
            first_section = false;
            continue;
        }

        if first_section {
            let segments = line.split(":").collect::<Vec<_>>();
            let wire = segments[0].to_string();
            let state = segments[1].trim();
            let state = match state {
                "1" => On,
                "0" => Off,
                _ => return Err(anyhow!("invalid wire state {}", state)),
            };
            wires.insert(wire, WireData::new(state));
        } else {
            let segments = line.split(" ").collect::<Vec<_>>();
            if segments.len() != 5 {
                return Err(anyhow!("invalid wire format"));
            }
            let in1 = segments[0].trim();
            let in2 = segments[2].trim();
            let out = segments[4].trim();
            update_wire_data(next_gate_id, in1, in2, out, &mut wires);

            let gate_type = match segments[1].trim() {
                "AND" => GateType::And,
                "OR" => GateType::Or,
                "XOR" => GateType::Xor,
                _ => return Err(anyhow!("invalid gate")),
            };
            gates.insert(next_gate_id, Gate::new(gate_type,
                                                 vec![in1.to_string(), in2.to_string()],
                                                 out.to_string()));
            next_gate_id += 1;
        }
    }


    Ok(Wiring::new(wires, gates))
}

fn update_wire_data(gate_id: GateId,
                    in1: &str,
                    in2: &str,
                    out: &str,
                    wires: &mut HashMap<String, WireData>) {

    let wire_data = wires
        .entry(in1.to_string())
        .or_insert(WireData::new(Unknown));
    wire_data.inputs.push(gate_id);

    let wire_data = wires
        .entry(in2.to_string())
        .or_insert(WireData::new(Unknown));
    wire_data.inputs.push(gate_id);

    let wire_data = wires
        .entry(out.to_string())
        .or_insert(WireData::new(Unknown));
    wire_data.output = Some(gate_id);
}