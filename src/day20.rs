// Day 20: Sprinklers
// https://lovemathboy.github.io/day20.html

use std::collections::HashSet;

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day20.txt").ok()?;

    Some((part1(&input).to_string(), part2(&input).to_string()))
}

#[derive(Debug, Clone, Copy)]
struct Circle {
    x: i64,
    y: i64,
    r: i64,
}

impl Circle {
    fn contains(&self, x: i64, y: i64) -> bool {
        (self.x - x).pow(2) + (self.y - y).pow(2) <= self.r.pow(2)
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) < (self.r + other.r).pow(2)
    }

    // Returns intersection points of the boundaries of two circles.
    fn intersections(&self, other: &Self) -> Option<((i64, i64), (i64, i64))> {
        let d2 = (self.x - other.x).pow(2) + (self.y - other.y).pow(2);
        let d = (d2 as u64).isqrt() as i64;

        if d > self.r + other.r || d < (self.r - other.r).abs() || d == 0 {
            return None;
        }

        let a = (self.r.pow(2) - other.r.pow(2) + d2) / (2 * d);
        let h = (self.r.pow(2) - a.pow(2)).max(0);
        let h = (h as u64).isqrt() as i64;

        let x2 = self.x + a * (other.x - self.x) / d;
        let y2 = self.y + a * (other.y - self.y) / d;

        let x3_1 = x2 + h * (other.y - self.y) / d;
        let y3_1 = y2 - h * (other.x - self.x) / d;

        let x3_2 = x2 - h * (other.y - self.y) / d;
        let y3_2 = y2 + h * (other.x - self.x) / d;

        Some(((x3_1, y3_1), (x3_2, y3_2)))
    }
}

fn parse_input(input: &str) -> Vec<Circle> {
    input
        .trim()
        .lines()
        .map(|line| {
            // Parse lines like "(4, 6) r=3"
            let (xy, r) = line.split_once(") r=").unwrap();
            let (x, y) = xy.trim_start_matches('(').split_once(", ").unwrap();
            Circle {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                r: r.parse().unwrap(),
            }
        })
        .collect()
}

fn part1(_input: &str) -> i64 {
    let sprinklers = parse_input(_input);

    let mut overlaps = vec![0; sprinklers.len()];

    for (idx, sprinkler) in sprinklers.iter().enumerate() {
        for (idx2, sprinkler2) in sprinklers.iter().enumerate().skip(idx + 1) {
            if sprinkler.overlaps(sprinkler2) {
                overlaps[idx] += 1;
                overlaps[idx2] += 1;
            }
        }
    }
    let max = overlaps
        .into_iter()
        .enumerate()
        .max_by_key(|(_, v)| *v)
        .unwrap();

    let max_sprinkler = &sprinklers[max.0];
    max_sprinkler.x * max_sprinkler.y + max.1
}

fn part2(input: &str) -> i64 {
    let sprinklers = parse_input(input);
    let mut candidates = HashSet::new();

    // Add centers
    for c in &sprinklers {
        candidates.insert((c.x, c.y));
    }

    // Add intersection points neighbors
    for (i, c1) in sprinklers.iter().enumerate() {
        for c2 in sprinklers.iter().skip(i + 1) {
            if let Some(points) = c1.intersections(c2) {
                candidates.insert(points.0);
                candidates.insert(points.1);
                // Check neighbors due to integer arithmetic truncation
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        candidates.insert((points.0.0 + dx, points.0.1 + dy));
                        candidates.insert((points.1.0 + dx, points.1.1 + dy));
                    }
                }
            }
        }
    }

    let mut max_covered = 0;
    let mut best_coord = (0, 0);

    for (cx, cy) in candidates {
        let mut count = 0;
        for s in &sprinklers {
            if s.contains(cx, cy) {
                count += 1;
            }
        }

        if count > max_covered {
            max_covered = count;
            best_coord = (cx, cy);
        }
    }

    best_coord.0 * best_coord.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
(4, 6) r=3
(3, 7) r=1
(12, 14) r=9
(10, 6) r=5
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 27);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 48);
    }
}
