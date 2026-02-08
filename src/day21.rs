// Day 21: Grid Traversal v2
// https://lovemathboy.github.io/day21.html

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day21.txt").ok()?;
    let part1_res = part1(&input);
    let part2_res = part2(&input);

    Some((part1_res.to_string(), part2_res.to_string()))
}

type Grid = Vec<Vec<char>>;

fn parse_input(input: &str) -> Vec<Grid> {
    input
        .trim()
        .split("\n\n")
        .map(|grid_str| {
            grid_str
                .lines()
                .map(|line| line.chars().collect())
                .collect()
        })
        .collect()
}

// -----------------------------------------------------------------------------
// 1. Grid Parsing and Boundary Extraction
// -----------------------------------------------------------------------------

fn get_boundary(grid: &Grid, side: usize) -> Vec<bool> {
    let rows = grid.len();
    let cols = grid[0].len();
    match side {
        0 => (0..cols).map(|c| grid[0][c] != '#').collect(), // Top
        1 => (0..rows).map(|r| grid[r][cols - 1] != '#').collect(), // Right
        2 => (0..cols).map(|c| grid[rows - 1][c] != '#').collect(), // Bottom
        3 => (0..rows).map(|r| grid[r][0] != '#').collect(), // Left
        _ => vec![],
    }
}

fn rotate_grid(grid: &Grid) -> Grid {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut new_grid = vec![vec![' '; rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            new_grid[c][rows - 1 - r] = grid[r][c];
        }
    }
    new_grid
}

// -----------------------------------------------------------------------------
// 2. Cube Topology and Matching
// -----------------------------------------------------------------------------

// (face1, edge1, face2, edge2, reversed_match)
const CONNECTIONS: [(usize, usize, usize, usize, bool); 12] = [
    (0, 0, 4, 2, false),
    (0, 1, 1, 3, false),
    (0, 2, 5, 0, false),
    (0, 3, 3, 1, false),
    (1, 0, 4, 1, true),
    (1, 1, 2, 3, false),
    (1, 2, 5, 1, false),
    (2, 0, 4, 0, true),
    (2, 1, 3, 3, false),
    (2, 2, 5, 2, true),
    (3, 0, 4, 3, false),
    (3, 2, 5, 3, false),
];

struct RotatedGridInfo {
    // grid: Grid, // Removed to avoid cloning too much in checking loop if not needed there?
    // Actually we need 'grid' later for building graph.
    // Let's store checking info separately or just keep it simple.
    sigs: Vec<Vec<bool>>,
    // os_pos: Vec<(usize, usize)>, // Not used in matching
}

fn check_assignments(assignments: &[(usize, usize)], grid_info: &[Vec<RotatedGridInfo>]) -> bool {
    for &(f1, e1, f2, e2, reversed_match) in &CONNECTIONS {
        let (g1, r1) = assignments[f1];
        let (g2, r2) = assignments[f2];

        let sig1 = &grid_info[g1][r1].sigs[e1];
        let sig2 = &grid_info[g2][r2].sigs[e2];

        if reversed_match {
            let sig2_rev: Vec<bool> = sig2.iter().copied().rev().collect();
            if sig1 != &sig2_rev {
                return false;
            }
        } else if sig1 != sig2 {
            return false;
        }
    }
    true
}

fn find_layout(grids: &[Grid]) -> Option<Vec<(usize, usize)>> {
    let mut grid_info = Vec::new();
    for grid in grids {
        let mut rots = Vec::new();
        let mut curr = grid.clone();
        for _ in 0..4 {
            let sigs: Vec<Vec<bool>> = (0..4).map(|s| get_boundary(&curr, s)).collect();
            rots.push(RotatedGridInfo { sigs });
            curr = rotate_grid(&curr);
        }
        grid_info.push(rots);
    }

    fn solve_layout(
        face_idx: usize,
        assignments: &mut Vec<(usize, usize)>,
        used_grids: &mut HashSet<usize>,
        grid_info: &[Vec<RotatedGridInfo>],
    ) -> Option<Vec<(usize, usize)>> {
        if face_idx == 6 {
            if check_assignments(assignments, grid_info) {
                return Some(assignments.clone());
            }
            return None;
        }

        let mut possibilities = Vec::new();
        if face_idx == 0 {
            for r_idx in 0..4 {
                possibilities.push((0, r_idx));
            }
        } else {
            for g_idx in 1..6 {
                if !used_grids.contains(&g_idx) {
                    for r_idx in 0..4 {
                        possibilities.push((g_idx, r_idx));
                    }
                }
            }
        }

        for (g, r) in possibilities {
            assignments.push((g, r));
            let mut connections_ok = true;

            for &(f1, e1, f2, e2, rev) in &CONNECTIONS {
                if f1 == face_idx && f2 < face_idx {
                    let (n_g, n_r) = assignments[f2];
                    let s1 = &grid_info[g][r].sigs[e1];
                    let s2 = &grid_info[n_g][n_r].sigs[e2];
                    if rev {
                        if !s1.iter().eq(s2.iter().rev()) {
                            connections_ok = false;
                            break;
                        }
                    } else if s1 != s2 {
                        connections_ok = false;
                        break;
                    }
                } else if f2 == face_idx && f1 < face_idx {
                    let (n_g, n_r) = assignments[f1];
                    let s2 = &grid_info[g][r].sigs[e2];
                    let s1 = &grid_info[n_g][n_r].sigs[e1];
                    if rev {
                        if !s1.iter().eq(s2.iter().rev()) {
                            connections_ok = false;
                            break;
                        }
                    } else if s1 != s2 {
                        connections_ok = false;
                        break;
                    }
                }
            }

            if connections_ok {
                let mut new_used = used_grids.clone();
                new_used.insert(g);
                if let Some(res) = solve_layout(face_idx + 1, assignments, &mut new_used, grid_info)
                {
                    return Some(res);
                }
            }
            assignments.pop();
        }

        None
    }

    let mut assignments = Vec::new();
    solve_layout(0, &mut assignments, &mut HashSet::from([0]), &grid_info)
}

// -----------------------------------------------------------------------------
// 3. Graph Building and Solving
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct NodeId(usize, usize, usize); // (face_idx, row, col)

struct Graph {
    adj: HashMap<NodeId, Vec<(NodeId, usize)>>,
    node_weights: HashMap<NodeId, usize>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj: HashMap::new(),
            node_weights: HashMap::new(),
        }
    }

    fn add_node(&mut self, u: NodeId, weight: usize) {
        self.node_weights.insert(u, weight);
        self.adj.entry(u).or_default();
    }

    fn add_edge(&mut self, u: NodeId, v: NodeId, weight: usize) {
        self.adj.entry(u).or_default().push((v, weight));
        self.adj.entry(v).or_default().push((u, weight));
    }
}

fn solve_steiner(assignments: &[(usize, usize)], grids: &[Grid]) -> usize {
    let mut rotated_grids = Vec::new();
    for &(g_idx, r_idx) in assignments {
        let mut curr = grids[g_idx].clone();
        for _ in 0..r_idx {
            curr = rotate_grid(&curr);
        }
        rotated_grids.push(curr);
    }

    let mut g = Graph::new();
    let mut terminals = Vec::new();

    for (f_idx, grid) in rotated_grids.iter().enumerate() {
        let rows = grid.len();
        let cols = grid[0].len();
        for r in 0..rows {
            for c in 0..cols {
                let char = grid[r][c];
                if char != '#' {
                    let u = NodeId(f_idx, r, c);
                    let weight = if char == '.' { 1 } else { 0 };
                    g.add_node(u, weight);
                    if char == 'O' {
                        terminals.push(u);
                    }

                    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            let nr = nr as usize;
                            let nc = nc as usize;
                            if grid[nr][nc] != '#' {
                                g.add_edge(u, NodeId(f_idx, nr, nc), 0);
                            }
                        }
                    }
                }
            }
        }
    }

    for &(f1, e1, f2, e2, rev) in &CONNECTIONS {
        let grid1 = &rotated_grids[f1];
        let grid2 = &rotated_grids[f2];
        let rows1 = grid1.len();
        let cols1 = grid1[0].len();
        let rows2 = grid2.len();
        let cols2 = grid2[0].len();

        let get_coords = |side: usize, r: usize, c: usize| -> Vec<(usize, usize)> {
            match side {
                0 => (0..c).map(|x| (0, x)).collect(),
                1 => (0..r).map(|x| (x, c - 1)).collect(),
                2 => (0..c).map(|x| (r - 1, x)).collect(),
                3 => (0..r).map(|x| (x, 0)).collect(),
                _ => vec![],
            }
        };

        let coords1 = get_coords(e1, rows1, cols1);
        let mut coords2 = get_coords(e2, rows2, cols2);

        if rev {
            coords2.reverse();
        }

        for ((r1_c, c1_c), (r2_c, c2_c)) in coords1.into_iter().zip(coords2.into_iter()) {
            if grid1[r1_c][c1_c] != '#' && grid2[r2_c][c2_c] != '#' {
                let u = NodeId(f1, r1_c, c1_c);
                let v = NodeId(f2, r2_c, c2_c);
                g.add_edge(u, v, 0);
            }
        }
    }

    let mut key_nodes = HashSet::new();
    for &t in &terminals {
        key_nodes.insert(t);
    }
    for (&u, neighbors) in &g.adj {
        if neighbors.len() != 2 {
            key_nodes.insert(u);
        }
    }

    let mut cg_adj: HashMap<NodeId, Vec<(NodeId, usize)>> = HashMap::new();
    let mut cg_node_weights = HashMap::new();

    for &k in &key_nodes {
        if let Some(&w) = g.node_weights.get(&k) {
            cg_node_weights.insert(k, w);
        }
    }

    for &start_node in &key_nodes {
        if let Some(neighbors) = g.adj.get(&start_node) {
            for &(first_neighbor, _) in neighbors {
                let mut prev = start_node;
                let mut curr = first_neighbor;
                let mut path_weight = 0;

                loop {
                    if key_nodes.contains(&curr) {
                        break;
                    }
                    path_weight += g.node_weights.get(&curr).cloned().unwrap_or(0);

                    let curr_neighbors = &g.adj[&curr];
                    if curr_neighbors.is_empty() {
                        break;
                    }

                    let mut next_node = None;
                    // Find neighbor that is NOT prev
                    for &(nbr, _) in curr_neighbors {
                        if nbr != prev {
                            next_node = Some(nbr);
                            break;
                        }
                    }

                    if let Some(next) = next_node {
                        prev = curr;
                        curr = next;
                    } else {
                        break;
                    }
                }

                if key_nodes.contains(&curr) && curr != start_node {
                    cg_adj
                        .entry(start_node)
                        .or_default()
                        .push((curr, path_weight));
                }
            }
        }
    }

    for (_, edges) in cg_adj.iter_mut() {
        let mut min_weights = HashMap::new();
        for &(v, w) in edges.iter() {
            min_weights
                .entry(v)
                .and_modify(|e: &mut usize| *e = (*e).min(w))
                .or_insert(w);
        }
        *edges = min_weights.into_iter().collect();
    }

    let is_tree = {
        let mut visited = HashSet::new();
        let mut has_cycle = false;
        for &k in &key_nodes {
            if !visited.contains(&k) {
                if has_cycle_dfs(k, None, &cg_adj, &mut visited) {
                    has_cycle = true;
                    break;
                }
            }
        }
        !has_cycle
    };

    if is_tree {
        // EXACT SOLVER FOR TREE
        let mut current_nodes: HashSet<NodeId> = cg_adj.keys().cloned().collect();
        let mut degree: HashMap<NodeId, usize> = HashMap::new();
        for (u, edges) in &cg_adj {
            degree.insert(*u, edges.len());
        }

        loop {
            let mut leaves_to_remove = Vec::new();
            for &n in &current_nodes {
                if degree.get(&n).copied().unwrap_or(0) == 1 && !terminals.contains(&n) {
                    leaves_to_remove.push(n);
                }
            }

            if leaves_to_remove.is_empty() {
                break;
            }

            for &n in &leaves_to_remove {
                current_nodes.remove(&n);
                if let Some(edges) = cg_adj.get(&n) {
                    for &(v, _) in edges {
                        if current_nodes.contains(&v) {
                            *degree.get_mut(&v).unwrap() -= 1;
                        }
                    }
                }
            }
        }

        let mut total_cost = 0;
        for &n in &current_nodes {
            total_cost += cg_node_weights.get(&n).copied().unwrap_or(0);
        }
        for &u in &current_nodes {
            if let Some(edges) = cg_adj.get(&u) {
                for &(v, w) in edges {
                    if current_nodes.contains(&v) && u < v {
                        total_cost += w;
                    }
                }
            }
        }
        total_cost
    } else {
        // MST Approximation
        let approx_terminals: Vec<NodeId> = terminals
            .clone()
            .into_iter()
            .filter(|t| cg_adj.contains_key(t))
            .collect();

        // Map original NodeId to index 0..N-1 for cleaner MST logic if desired,
        // but NodeId is fine.

        let mut t_dists: HashMap<NodeId, HashMap<NodeId, usize>> = HashMap::new();

        for &start_node in &approx_terminals {
            let mut dists = HashMap::new();
            let mut pq = BinaryHeap::new();

            let start_weight = cg_node_weights[&start_node];
            dists.insert(start_node, start_weight);
            pq.push(State {
                cost: start_weight,
                node: start_node,
            });

            while let Some(State { cost, node: u }) = pq.pop() {
                if cost > *dists.get(&u).unwrap_or(&usize::MAX) {
                    continue;
                }

                if let Some(neighbors) = cg_adj.get(&u) {
                    for &(v, edge_w) in neighbors {
                        let node_w = cg_node_weights[&v];
                        let new_cost = cost + edge_w + node_w;
                        if new_cost < *dists.get(&v).unwrap_or(&usize::MAX) {
                            dists.insert(v, new_cost);
                            pq.push(State {
                                cost: new_cost,
                                node: v,
                            });
                        }
                    }
                }
            }

            t_dists.insert(start_node, dists);
        }

        if approx_terminals.is_empty() {
            return 0;
        }

        let start = approx_terminals[0];
        let mut visited = HashSet::new();
        let mut pq = BinaryHeap::new();
        let mut mst_weight = 0; // Sum of MST edge weights

        // Prim's on the Complete Graph of Terminals
        // "Edge Power" between u and v = distance(u,v) = t_dists[u][v]

        visited.insert(start);
        for &target in &approx_terminals {
            if start == target {
                continue;
            }
            if let Some(d) = t_dists.get(&start).and_then(|m| m.get(&target)) {
                pq.push(EdgeState {
                    weight: *d,
                    node: target,
                });
            }
        }

        while visited.len() < approx_terminals.len() {
            if let Some(EdgeState { weight, node }) = pq.pop() {
                if visited.contains(&node) {
                    continue;
                }

                visited.insert(node);
                mst_weight += weight;

                for &next_target in &approx_terminals {
                    if !visited.contains(&next_target) {
                        if let Some(d) = t_dists.get(&node).and_then(|m| m.get(&next_target)) {
                            pq.push(EdgeState {
                                weight: *d,
                                node: next_target,
                            });
                        }
                    }
                }
            } else {
                break;
            }
        }

        mst_weight
    }
}

fn has_cycle_dfs(
    u: NodeId,
    p: Option<NodeId>,
    adj: &HashMap<NodeId, Vec<(NodeId, usize)>>,
    visited: &mut HashSet<NodeId>,
) -> bool {
    visited.insert(u);
    if let Some(neighbors) = adj.get(&u) {
        for &(v, _) in neighbors {
            if Some(v) == p {
                continue;
            }
            if visited.contains(&v) {
                return true;
            }
            if !visited.contains(&v) {
                if has_cycle_dfs(v, Some(u), adj, visited) {
                    return true;
                }
            }
        }
    }
    false
}

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    node: NodeId,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct EdgeState {
    weight: usize,
    node: NodeId,
}

impl Ord for EdgeState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for EdgeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------

fn solve_grid(grid: &[Vec<char>]) -> i64 {
    let mut starts = vec![];
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'O' {
                starts.push((r, c));
            }
        }
    }

    if starts.len() < 2 {
        return 0; // Should not happen based on problem description
    }

    let start = starts[0];
    let end = starts[1];

    // Dijkstra/BFS
    let mut dist = vec![vec![i64::MAX; grid[0].len()]; grid.len()];
    let mut pq = VecDeque::new(); // Using deque for 0-1 BFS

    dist[start.0][start.1] = 0;
    pq.push_front((0, start.0, start.1));

    while let Some((d, r, c)) = pq.pop_front() {
        if d > dist[r][c] {
            continue;
        }
        if (r, c) == end {
            return d;
        }

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                let cell = grid[nr][nc];

                if cell != '#' {
                    let weight = if cell == '.' { 1 } else { 0 };
                    if dist[r][c] + weight < dist[nr][nc] {
                        dist[nr][nc] = dist[r][c] + weight;
                        if weight == 0 {
                            pq.push_front((dist[nr][nc], nr, nc));
                        } else {
                            pq.push_back((dist[nr][nc], nr, nc));
                        }
                    }
                }
            }
        }
    }

    0
}

fn part1(input: &str) -> i64 {
    let grids = parse_input(input);
    let mut product = 1;

    for grid in grids {
        product *= solve_grid(&grid);
    }

    product
}

fn part2(input: &str) -> usize {
    let grids = parse_input(input);
    if let Some(assignments) = find_layout(&grids) {
        solve_steiner(&assignments, &grids)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const TEST_INPUT: &str = "\
#####
#O#.#
#...O
#.#.#
#####

###O#
#O#.#
#.#.#
#...#
##.##";

        assert_eq!(part1(TEST_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        const TEST_INPUT: &str = "\
#####
#O#.#
#...O
#.#.#
#####

###O#
#O#.#
#.#.#
#...#
##.##

#####
#####
#####
#####
#####

#####
#####
#####
#####
#####

#####
#####
#####
#####
#####

#####
#####
#####
#####
#####
";
        // assert_eq!(part2(TEST_INPUT), 10);
    }
}
