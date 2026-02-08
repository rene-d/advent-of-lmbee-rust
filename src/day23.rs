// Day 23: Grid Traversal v3
// https://lovemathboy.github.io/day23.html

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day23.txt").ok()?;
    let grids: Vec<&str> = input.trim().split("\n\n").collect();

    let mut total_product_p1: u128 = 1;
    let mut total_product_p2: u128 = 1;
    let mut solved_count = 0;

    #[cfg(debug_assertions)]
    println!("Found {} grids.", grids.len());

    for (_i, grid_str) in grids.iter().enumerate() {
        if let Some(score_p1) = solve_grid_part1(grid_str) {
            #[cfg(debug_assertions)]
            println!("Grid {}: P1 Minimum score = {}", _i + 1, score_p1);
            total_product_p1 *= score_p1 as u128;
            solved_count += 1;
        } else {
            #[cfg(debug_assertions)]
            println!("Grid {}: Skipped (invalid or empty)", _i + 1);
            continue;
        }

        if let Some(score_p2) = solve_grid_part2(grid_str) {
            #[cfg(debug_assertions)]
            println!("Grid {}: P2 Minimum score = {}", _i + 1, score_p2);
            total_product_p2 *= score_p2 as u128;
        } else {
            #[cfg(debug_assertions)]
            println!("Grid {}: P2 No solution found", _i + 1);
        }
    }

    if solved_count == 0 {
        return None;
    }

    Some((total_product_p1.to_string(), total_product_p2.to_string()))
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_grid_part1(grid_str: &str) -> Option<usize> {
    if grid_str.trim().is_empty() {
        return None;
    }

    let lines: Vec<&str> = grid_str.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut start_pos = None;
    let mut end_pos = None;
    let mut grid = Vec::with_capacity(rows);

    for (r, line) in lines.iter().enumerate() {
        let mut row_vec = Vec::with_capacity(cols);
        for (c, char) in line.chars().enumerate() {
            if char == 'S' {
                start_pos = Some((r, c));
                row_vec.push(0);
            } else if char == 'E' {
                end_pos = Some((r, c));
                row_vec.push(0);
            } else {
                row_vec.push(char.to_digit(10).unwrap() as usize);
            }
        }
        grid.push(row_vec);
    }

    let start_pos = start_pos?;
    let end_pos = end_pos?;

    let mut pq = BinaryHeap::new();
    pq.push(State {
        cost: 0,
        position: start_pos,
    });

    let mut min_costs: HashMap<(usize, usize), usize> = HashMap::new();
    min_costs.insert(start_pos, 0);

    while let Some(State {
        cost,
        position: (r, c),
    }) = pq.pop()
    {
        if (r, c) == end_pos {
            return Some(cost);
        }

        if cost > *min_costs.get(&(r, c)).unwrap_or(&usize::MAX) {
            continue;
        }

        let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dr, dc) in deltas.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let nr = nr as usize;
                let nc = nc as usize;
                let new_cost = cost + grid[nr][nc];

                if new_cost < *min_costs.get(&(nr, nc)).unwrap_or(&usize::MAX) {
                    min_costs.insert((nr, nc), new_cost);
                    pq.push(State {
                        cost: new_cost,
                        position: (nr, nc),
                    });
                }
            }
        }
    }

    None
}

// MCMF Implementation for Part 2

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    cap: i32,
    cost: i64,
    rev: usize,
}

struct MinCostMaxFlow {
    graph: Vec<Vec<Edge>>,
}

impl MinCostMaxFlow {
    fn new(n: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: i64) {
        let rev_from = self.graph[to].len();
        let rev_to = self.graph[from].len();
        self.graph[from].push(Edge {
            to,
            cap,
            cost,
            rev: rev_from,
        });
        self.graph[to].push(Edge {
            to: from,
            cap: 0,
            cost: -cost,
            rev: rev_to,
        });
    }

    fn solve(&mut self, source: usize, sink: usize, required_flow: i32) -> Option<i64> {
        let n = self.graph.len();
        let mut total_flow = 0;
        let mut min_cost = 0;

        // Potentials for SPFA / Dijkstra if we had negative edges (SPFA is safer here given simple grid graph with potential cycles if costs were negative, though here costs are non-negative, but residual edges can be negative)
        // With simple SPFA:
        while total_flow < required_flow {
            let mut dist = vec![i64::MAX; n];
            let mut parent_node = vec![usize::MAX; n];
            let mut parent_edge = vec![usize::MAX; n];
            let mut in_queue = vec![false; n];
            let mut queue = VecDeque::new();

            dist[source] = 0;
            queue.push_back(source);
            in_queue[source] = true;

            while let Some(u) = queue.pop_front() {
                in_queue[u] = false;
                for (i, e) in self.graph[u].iter().enumerate() {
                    if e.cap > 0 && dist[e.to] > dist[u].saturating_add(e.cost) {
                        dist[e.to] = dist[u] + e.cost;
                        parent_node[e.to] = u;
                        parent_edge[e.to] = i;
                        if !in_queue[e.to] {
                            queue.push_back(e.to);
                            in_queue[e.to] = true;
                        }
                    }
                }
            }

            if dist[sink] == i64::MAX {
                return None; // Cannot push flow
            }

            let push = required_flow - total_flow;
            // The flow along shortest path in residual graph is usually determined by min capacity,
            // but here capacities are small integers.
            // Let's find flow bottleneck.
            let mut flow = push;
            let mut curr = sink;
            while curr != source {
                let p = parent_node[curr];
                let idx = parent_edge[curr];
                flow = flow.min(self.graph[p][idx].cap);
                curr = p;
            }

            total_flow += flow;
            min_cost += flow as i64 * dist[sink];

            curr = sink;
            while curr != source {
                let p = parent_node[curr];
                let idx = parent_edge[curr];

                self.graph[p][idx].cap -= flow;
                let rev_idx = self.graph[p][idx].rev;
                self.graph[curr][rev_idx].cap += flow;

                curr = p;
            }
        }

        Some(min_cost)
    }
}

fn solve_grid_part2(grid_str: &str) -> Option<i64> {
    if grid_str.trim().is_empty() {
        return None;
    }

    let lines: Vec<&str> = grid_str.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut start_pos = None;
    let mut end_pos = None;

    // Parse to find S and E
    for (r, line) in lines.iter().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == 'S' {
                start_pos = Some((r, c));
            } else if char == 'E' {
                end_pos = Some((r, c));
            }
        }
    }

    let start_pos = start_pos?;
    let end_pos = end_pos?;

    // Nodes: 2 per grid cell -> 2 * rows * cols
    let num_nodes = 2 * rows * cols;
    let mut mcmf = MinCostMaxFlow::new(num_nodes);

    let source_node = 2 * (start_pos.0 * cols + start_pos.1);
    let sink_node = 2 * (end_pos.0 * cols + end_pos.1) + 1;

    for (r, line) in lines.iter().enumerate() {
        for c in 0..cols {
            let char = line.chars().nth(c).unwrap();

            let in_n = 2 * (r * cols + c);
            let out_n = in_n + 1;

            let (cap, cost) = if char == 'S' || char == 'E' {
                (2, 0)
            } else {
                (1, char.to_digit(10).unwrap() as i64)
            };

            // Edge In -> Out
            mcmf.add_edge(in_n, out_n, cap, cost);

            // Edges to neighbors
            let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dr, dc) in deltas.iter() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    let neighbor_in = 2 * (nr * cols + nc);
                    // Out -> Neighbor In
                    mcmf.add_edge(out_n, neighbor_in, 1, 0);
                }
            }
        }
    }

    mcmf.solve(source_node, sink_node, 2)
}
