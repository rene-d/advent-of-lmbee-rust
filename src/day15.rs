// Day 15: Mediocre Toys
// https://lovemathboy.github.io/day15.html

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day15.txt").ok()?;
    Some((part1(&input).to_string(), part2(&input).to_string()))
}

fn part1(input: &str) -> u64 {
    let mut lower: BinaryHeap<i64> = BinaryHeap::new();
    let mut upper: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut satisfaction_score = 0;
    let mut request_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("receive") {
            let val_str = line.strip_prefix("receive ").unwrap();
            let val: i64 = val_str.parse().unwrap();

            // Always push to lower (max-heap) first
            lower.push(val);

            // Move max of lower to upper (min-heap)
            if let Some(max_lower) = lower.pop() {
                upper.push(Reverse(max_lower));
            }

            // Balance sizes: lower can have at most 1 element more than upper
            if upper.len() > lower.len()
                && let Some(Reverse(min_upper)) = upper.pop()
            {
                lower.push(min_upper);
            }
        } else if line == "request" {
            request_count += 1;
            let median = if lower.len() > upper.len() {
                lower.pop().unwrap()
            } else {
                let Reverse(val) = upper.pop().unwrap();
                val
            };

            satisfaction_score += request_count * (median as u64);
        }
    }

    satisfaction_score
}

fn part2(input: &str) -> u64 {
    // (quality, count)
    // lower is a max-heap (by quality)
    let mut lower: BinaryHeap<(i64, u64)> = BinaryHeap::new();
    // upper is a min-heap (by quality), using Reverse
    let mut upper: BinaryHeap<Reverse<(i64, u64)>> = BinaryHeap::new();

    let mut count_lower = 0;
    let mut count_upper = 0;

    let mut satisfaction_score = 0;
    let mut request_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("receive") {
            let val_str = line.strip_prefix("receive ").unwrap();
            let val: i64 = val_str.parse().unwrap();
            let count = val as u64; // Receive x copies

            // Determine where to add
            let add_to_lower = if let Some((max_lower, _)) = lower.peek() {
                val <= *max_lower
            } else {
                true
            };

            if add_to_lower {
                lower.push((val, count));
                count_lower += count;
            } else {
                upper.push(Reverse((val, count)));
                count_upper += count;
            }

            // Rebalance
            let total = count_lower + count_upper;
            let target_lower = total.div_ceil(2);

            while count_lower < target_lower {
                let diff = target_lower - count_lower;
                let Reverse((u_val, u_count)) = upper.pop().unwrap();

                if u_count <= diff {
                    lower.push((u_val, u_count));
                    count_lower += u_count;
                    count_upper -= u_count;
                } else {
                    lower.push((u_val, diff));
                    upper.push(Reverse((u_val, u_count - diff)));
                    count_lower += diff;
                    count_upper -= diff;
                }
            }

            while count_lower > target_lower {
                let diff = count_lower - target_lower;
                let (l_val, l_count) = lower.pop().unwrap();

                if l_count <= diff {
                    upper.push(Reverse((l_val, l_count)));
                    count_lower -= l_count;
                    count_upper += l_count;
                } else {
                    upper.push(Reverse((l_val, diff)));
                    lower.push((l_val, l_count - diff));
                    count_lower -= diff;
                    count_upper += diff;
                }
            }
        } else if line == "request" {
            request_count += 1;
            let total = count_lower + count_upper;
            if total == 0 {
                continue;
            }

            let median_val;

            if count_lower > count_upper {
                // Median in lower
                let mut top = lower.peek_mut().unwrap();
                median_val = top.0;
                if top.1 > 1 {
                    top.1 -= 1;
                } else {
                    std::collections::binary_heap::PeekMut::pop(top);
                }
                count_lower -= 1;
            } else {
                // Median in upper
                let mut top = upper.peek_mut().unwrap();
                median_val = (top.0).0;
                if (top.0).1 > 1 {
                    (top.0).1 -= 1;
                } else {
                    std::collections::binary_heap::PeekMut::pop(top);
                }
                count_upper -= 1;
            }

            satisfaction_score += request_count * (median_val as u64);

            // Rebalance after removal
            let total = count_lower + count_upper;
            let target_lower = total.div_ceil(2);

            while count_lower > target_lower {
                let diff = count_lower - target_lower;
                let (l_val, l_count) = lower.pop().unwrap();
                if l_count <= diff {
                    upper.push(Reverse((l_val, l_count)));
                    count_lower -= l_count;
                    count_upper += l_count;
                } else {
                    upper.push(Reverse((l_val, diff)));
                    lower.push((l_val, l_count - diff));
                    count_lower -= diff;
                    count_upper += diff;
                }
            }

            // Note: If count_lower < target_lower, we might need to move from upper to lower.
            // Example: L=3 (target 3), R=2. Total 5. Removed from L.
            // New L=2, R=2. Total 4. Target 2. OK.
            // Example: L=2 (target 2), R=2. Total 4. Removed from R.
            // New L=2, R=1. Total 3. Target 2. OK.

            // Wait, what if L=3, R=3. Target 3. Remove from R.
            // New L=3, R=2. Total 5. Target 3. OK.

            // What if L=3, R=2. Remove from L.
            // New L=2, R=2 (Valid state).

            // However, inserting might leave us with slight imbalance that only gets fixed one way?
            // "count_lower < target_lower" check:
            while count_lower < target_lower {
                let diff = target_lower - count_lower;
                // Check if upper has elements to give?
                if let Some(Reverse((u_val, u_count))) = upper.pop() {
                    if u_count <= diff {
                        lower.push((u_val, u_count));
                        count_lower += u_count;
                        count_upper -= u_count;
                    } else {
                        lower.push((u_val, diff));
                        upper.push(Reverse((u_val, u_count - diff)));
                        count_lower += diff;
                        count_upper -= diff;
                    }
                } else {
                    break;
                }
            }
        }
    }

    satisfaction_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
receive 4
receive 9
receive 2
receive 9
request
request
receive 99999
request
request";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 400040);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 700020);
    }
}
