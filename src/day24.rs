// Day 24: Pyramid
// https://lovemathboy.github.io/day24.html

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day24.txt").ok()?;
    Some((part1(&input).to_string(), part2(&input).to_string()))
}

fn part1(input: &str) -> u64 {
    let mut a: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut r = 0;
    while !a.is_empty() {
        r += a.iter().sum::<u64>();
        let mut b = Vec::with_capacity(a.len().saturating_sub(1));
        for i in 0..a.len().saturating_sub(1) {
            b.push(a[i].max(a[i + 1]) + 1);
        }
        a = b;
    }
    r
}

fn part2(input: &str) -> u64 {
    let raw_nums: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut a = Vec::new();

    // Decompress
    for chunk in raw_nums.chunks(5) {
        if chunk.len() < 5 {
            break;
        }
        let initial = chunk[0];
        let b = chunk[1];
        let c = chunk[2];
        let mod_val = chunk[3];
        let n = chunk[4];

        let mut current = initial;
        a.push(current);
        for _ in 1..n {
            current = (b * current + c) % mod_val;
            a.push(current);
        }
    }

    let n = a.len();

    // Term 1: Sum of heights
    // sum_{h=0}^{N-1} h * (N - h)
    let mut term1 = 0;
    for h in 0..n {
        term1 += (h as u64) * ((n - h) as u64);
    }

    // Term 2: Sum of max(subarray)
    // Left bound: index of previous element >= A[i] (so strictly less to left)

    let mut left_bound = vec![-1i64; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if a[top] < a[i] {
                stack.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stack.last() {
            left_bound[i] = top as i64;
        } else {
            left_bound[i] = -1;
        }
        stack.push(i);
    }

    // Right bound: index of next element > A[i] (so <= to right)

    let mut right_bound = vec![n as i64; n];
    stack.clear();

    for i in (0..n).rev() {
        while let Some(&top) = stack.last() {
            if a[top] <= a[i] {
                stack.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stack.last() {
            right_bound[i] = top as i64;
        } else {
            right_bound[i] = n as i64;
        }
        stack.push(i);
    }

    let mut term2 = 0;
    for i in 0..n {
        let l = left_bound[i];
        let r = right_bound[i];
        let count = ((i as i64 - l) * (r - i as i64)) as u64;
        term2 += count * a[i];
    }

    term1 + term2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "4 9 2 7 9";
        assert_eq!(part1(input), 139);
    }

    #[test]
    fn test_part2() {
        let input = "4 9 2 7 9 1 2 3 61 5";
        assert_eq!(part2(input), 1618);
    }
}
