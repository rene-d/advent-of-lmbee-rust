// Day 17: Networking
// https://lovemathboy.github.io/day17.html

use std::collections::HashMap;

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day17.txt").ok()?;
    Some((part1(&input).to_string(), part2(&input).to_string()))
}

fn parse_input(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut edges = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once(" -> ").unwrap();
        let from = from.parse::<u32>().unwrap();
        let to = to.parse::<u32>().unwrap();
        edges.entry(from).or_insert_with(Vec::new).push(to);
    }

    edges
}

fn part1(input: &str) -> u32 {
    let edges = parse_input(input);
    let mut memo = HashMap::new();
    let mut max_len = 0;

    for &start_node in edges.keys() {
        max_len = max_len.max(longest_path(start_node, &edges, &mut memo));
    }

    max_len
}

fn longest_path(node: u32, edges: &HashMap<u32, Vec<u32>>, memo: &mut HashMap<u32, u32>) -> u32 {
    if let Some(&len) = memo.get(&node) {
        return len;
    }

    let mut max_depth = 0;
    if let Some(neighbors) = edges.get(&node) {
        for &neighbor in neighbors {
            max_depth = max_depth.max(longest_path(neighbor, edges, memo));
        }
    }

    let result = 1 + max_depth; // 1 for current node
    memo.insert(node, result);
    result
}

fn part2(input: &str) -> u64 {
    let edges = parse_input(input);
    let mut nodes = std::collections::HashSet::new();
    for (&u, neighbors) in &edges {
        nodes.insert(u);
        for &v in neighbors {
            nodes.insert(v);
        }
    }

    let mut ids: HashMap<u32, u32> = HashMap::new();
    let mut low: HashMap<u32, u32> = HashMap::new();
    let mut on_stack: HashMap<u32, bool> = HashMap::new();
    let mut stack: Vec<u32> = Vec::new();

    let mut id_counter = 0;
    let mut scc_count = 0;
    let mut node_scc: HashMap<u32, u32> = HashMap::new(); // Map node -> scc_id

    // Sort nodes for deterministic behavior if needed, or just iterate
    for &node in &nodes {
        if !ids.contains_key(&node) {
            dfs(
                node,
                &edges,
                &mut ids,
                &mut low,
                &mut on_stack,
                &mut stack,
                &mut id_counter,
                &mut scc_count,
                &mut node_scc,
            );
        }
    }

    if scc_count == 1 {
        return 0;
    }

    // Calculate in/out degrees of SCCs
    let mut scc_in_degree = vec![0; scc_count];
    let mut scc_out_degree = vec![0; scc_count];

    for (&u, neighbors) in &edges {
        let u_scc = node_scc[&u] as usize;
        for &v in neighbors {
            let v_scc = node_scc[&v] as usize;
            if u_scc != v_scc {
                scc_out_degree[u_scc] += 1;
                scc_in_degree[v_scc] += 1;
            }
        }
    }

    let sources = scc_in_degree.iter().filter(|&&d| d == 0).count();
    let sinks = scc_out_degree.iter().filter(|&&d| d == 0).count();

    sources.max(sinks) as u64
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    at: u32,
    edges: &HashMap<u32, Vec<u32>>,
    ids: &mut HashMap<u32, u32>,
    low: &mut HashMap<u32, u32>,
    on_stack: &mut HashMap<u32, bool>,
    stack: &mut Vec<u32>,
    id_counter: &mut u32,
    scc_count: &mut usize,
    node_scc: &mut HashMap<u32, u32>,
) {
    stack.push(at);
    on_stack.insert(at, true);
    ids.insert(at, *id_counter);
    low.insert(at, *id_counter);
    *id_counter += 1;

    if let Some(neighbors) = edges.get(&at) {
        for &to in neighbors {
            if !ids.contains_key(&to) {
                dfs(
                    to, edges, ids, low, on_stack, stack, id_counter, scc_count, node_scc,
                );
                let low_at = *low.get(&at).unwrap();
                let low_to = *low.get(&to).unwrap();
                low.insert(at, low_at.min(low_to));
            } else if *on_stack.get(&to).unwrap_or(&false) {
                let low_at = *low.get(&at).unwrap();
                let id_to = *ids.get(&to).unwrap();
                low.insert(at, low_at.min(id_to));
            }
        }
    }

    if ids[&at] == low[&at] {
        while let Some(node) = stack.pop() {
            on_stack.insert(node, false);
            node_scc.insert(node, *scc_count as u32);
            if node == at {
                break;
            }
        }
        *scc_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
0 -> 3
1 -> 2
2 -> 3
3 -> 5
1 -> 3
2 -> 6
4 -> 7
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 3);
    }
}
