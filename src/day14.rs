// Day 14: Flip-Flop
// https://lovemathboy.github.io/day14.html

use std::collections::HashMap;

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day14.txt").ok()?;

    Some((part1(&input).to_string(), part2(&input).to_string()))
}

fn part1(input: &str) -> u32 {
    let mut circuit = Circuit::from(input);
    circuit.run(123456)
}

fn part2(input: &str) -> u128 {
    let circuit = Circuit::from(input);
    circuit.solve(12, 3456, 10u128.pow(15))
}

type SignalId = usize;

const SIGNAL_INP: SignalId = 0;
const SIGNAL_OUT: SignalId = 1;
const SIGNAL_BIN: SignalId = 2;

#[derive(Clone)]
struct FlipFlop {
    state: bool, // false => output1, true => output2
    outputs: [SignalId; 2],
}

impl FlipFlop {
    fn run(&mut self) -> SignalId {
        let next = self.outputs[usize::from(self.state)];
        self.state = !self.state;
        next
    }
}

struct Mapping<'a> {
    map: HashMap<&'a str, SignalId>,
    next_id: SignalId,
}

impl<'a> Mapping<'a> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            next_id: 3,
        }
    }

    fn size(&self) -> usize {
        self.next_id + 3
    }

    fn signal_id(&mut self, name: &'a str) -> SignalId {
        match name {
            "INP" => SIGNAL_INP,
            "OUT" => SIGNAL_OUT,
            "BIN" => SIGNAL_BIN,
            _ => {
                if let Some(id) = self.map.get(name) {
                    *id
                } else {
                    let id = self.next_id;
                    self.map.insert(name, id);
                    self.next_id += 1;
                    id
                }
            }
        }
    }
}

struct Circuit {
    circuit: Vec<FlipFlop>,
    start: SignalId,
}

impl Circuit {
    fn from(input: &str) -> Self {
        let mut signals = Mapping::new();

        let mut circuit = Vec::new();
        let mut start = 0;

        for line in input.lines() {
            let (src, b) = line.split_once(':').unwrap();

            let src_id = signals.signal_id(src);

            if src_id == SIGNAL_INP {
                let wire = b.trim_ascii();
                start = signals.signal_id(wire);
            } else {
                let (flip, flop) = b.trim_ascii().split_once(' ').unwrap();

                let flip_id = signals.signal_id(flip);
                let flop_id = signals.signal_id(flop);

                circuit.resize(
                    signals.size(),
                    FlipFlop {
                        state: false,
                        outputs: [0, 0],
                    },
                );

                circuit[src_id].outputs[0] = flip_id;
                circuit[src_id].outputs[1] = flop_id;
            }
        }

        Self { circuit, start }
    }

    fn run(&mut self, n: usize) -> u32 {
        let mut count = 0;
        for _ in 0..n {
            let mut signal = self.start;

            loop {
                signal = self.circuit[signal].run();
                if signal == SIGNAL_BIN {
                    break;
                }
                if signal == SIGNAL_OUT {
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    fn topo_from_inp(&self) -> Vec<SignalId> {
        let mut state = HashMap::new();
        let mut order = Vec::new();
        let mut stack = Vec::new();
        stack.push((self.start, 0));

        while let Some((node, phase)) = stack.pop() {
            if state.get(&node) == Some(&2) {
                continue;
            }

            if phase == 1 {
                state.insert(node, 2);
                order.push(node);
                continue;
            }

            if state.get(&node) == Some(&1) {
                continue;
            }
            state.insert(node, 1);
            stack.push((node, 1));

            for child in self.circuit[node].outputs.iter() {
                if *child == SIGNAL_BIN || *child == SIGNAL_OUT {
                    continue;
                }
                if state.get(child) != Some(&2) {
                    stack.push((*child, 0));
                }
            }
        }
        order.reverse();
        order
    }

    fn compute_depths_to_out(&self, order: &[SignalId]) -> HashMap<SignalId, usize> {
        let mut depth = HashMap::new();
        for node in order.iter().rev() {
            let mut max_len = 0;
            for child in self.circuit[*node].outputs.iter() {
                if *child == SIGNAL_OUT {
                    max_len = max_len.max(1);
                } else {
                    let d = depth.get(child).unwrap_or(&0);
                    if *d > 0 {
                        max_len = max_len.max(d + 1);
                    }
                }
            }
            depth.insert(*node, max_len);
        }
        depth
    }

    fn solve(&self, base: u128, exp: u32, modulus: u128) -> u128 {
        let order = self.topo_from_inp();

        let depth = self.compute_depths_to_out(&order);

        let mut mod_map = HashMap::new();
        for (node, d) in depth.iter() {
            if *d > 0 {
                let zz = modulus << d;
                if zz >> d != modulus {
                    panic!("overflow");
                }
                mod_map.insert(*node, modulus << d);
            }
        }

        let mut counts = HashMap::new();
        let mut out_count = 0;

        counts.insert(self.start, total_mod(base, exp, mod_map[&self.start]));

        for node in order {
            let n_mod = counts[&node] % mod_map[&node];
            for (idx, child) in self.circuit[node].outputs.iter().enumerate() {
                if *child == SIGNAL_OUT {
                    let send = split_mod(n_mod, modulus, idx == 0);
                    out_count = (out_count + send) % modulus;
                } else if *child == SIGNAL_BIN {
                    continue;
                } else {
                    let mod_child = mod_map[child];
                    let send = split_mod(n_mod, mod_child, idx == 0);
                    counts.insert(*child, (counts.get(child).unwrap_or(&0) + send) % mod_child);
                }
            }
        }
        out_count
    }
}

fn total_mod(base: u128, exp: u32, modulus: u128) -> u128 {
    mod_pow_safe(base, exp, modulus)
}

fn split_mod(n_mod: u128, mod_target: u128, pick_first: bool) -> u128 {
    let base = mod_target * 2;
    let n_reduced = n_mod % base;
    let q = n_reduced / 2;
    let r = n_reduced % 2;
    if pick_first {
        (q + r) % mod_target
    } else {
        q % mod_target
    }
}

// Multiplication modulaire (a * b) % m sans overflow u128
fn mul_mod(mut a: u128, mut b: u128, m: u128) -> u128 {
    let mut res = 0;
    a %= m;
    while b > 0 {
        if b % 2 == 1 {
            res = (res + a) % m; // Addition au lieu de multiplication
        }
        a = (a * 2) % m; // Doublement au lieu de carré
        b /= 2;
    }
    res
}

fn mod_pow_safe(mut base: u128, mut exp: u32, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exp /= 2;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
INP: abc
abc: def ghi
def: OUT ghi
ghi: OUT BIN
";

    #[test]
    fn test_part1() {
        let mut circuit = Circuit::from(TEST_INPUT);
        assert_eq!(circuit.run(6), 4);
    }

    #[test]
    fn test_part2() {
        let circuit = Circuit::from(TEST_INPUT);
        assert_eq!(circuit.solve(12, 3456, 10u128.pow(15)), 660414548213760);
    }

    #[test]
    fn test_total_mod() {
        assert_eq!(
            total_mod(12, 3456, 562949953421312000000000000000),
            444320153330481056663277142016
        );
    }
}
