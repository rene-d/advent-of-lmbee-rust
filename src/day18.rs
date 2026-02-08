// Day 18: Pattern Matching
// https://lovemathboy.github.io/day18.html

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day18.txt").ok()?;
    Some((part1(&input).to_string(), part2(&input).to_string()))
}

fn parse_input(input: &str) -> (Vec<&str>, &str) {
    let mut lines = input.lines();
    let mut patterns = Vec::new();
    let mut target = "";

    // Skip "Pattern:"
    if let Some(first) = lines.next()
        && first.trim() != "Pattern:"
    {
        // Handle case where "Pattern:" might be missing or different
        // For now assume standard format, but maybe print warning if not
    }

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "String:" {
            if let Some(t) = lines.next() {
                target = t.trim();
            }
            break;
        }
        patterns.push(line);
    }
    (patterns, target)
}

fn part1(input: &str) -> usize {
    let (patterns, target) = parse_input(input);
    patterns
        .iter()
        .filter(|pattern| matches(pattern, target))
        .count()
}

fn matches(pattern: &str, target: &str) -> bool {
    if pattern.len() != target.len() {
        return false;
    }
    pattern
        .chars()
        .zip(target.chars())
        .all(|(p, t)| p == '?' || p == t)
}

fn part2(input: &str) -> usize {
    let (patterns, target) = parse_input(input);

    let pattern = patterns.join("");
    let n = pattern.len();
    let m = target.len();

    if n < m {
        return 0;
    }

    // `matches_at[i]` is true if `target` matches `pattern[i..i+m]`
    let mut matches_at = vec![false; n - m + 1];
    for i in 0..=n - m {
        if matches(&pattern[i..i + m], target) {
            matches_at[i] = true;
        }
    }

    // `compatible_shift[d]` is true if `target` overlaps consistently with itself shifted by `d`.
    // Check target[k] == target[k + d] for all valid k.
    let mut compatible_shift = vec![false; m];
    for (d, compatible) in compatible_shift.iter_mut().enumerate().skip(1) {
        *compatible =
            (0..m - d).all(|k| target.chars().nth(k) == target.chars().nth(k + d));
    }

    // dp[i] = max occurrences for suffix i..end
    // count_starting_at[i] = max occurrences for suffix i..end PROVIDED we take a match at i.
    let mut dp = vec![0; n + 2];
    let mut count_starting_at = vec![0; n + 2];

    for i in (0..=n - m).rev() {
        if matches_at[i] {
            // Option: Overlapping next match
            let mut max_next = dp[std::cmp::min(i + m, n + 1)]; // Default: next match non-overlapping

            for d in 1..m {
                if i + d <= n - m && compatible_shift[d] {
                    // If compatible, we can potentially chain with a match starting strictly at i+d
                    if matches_at[i + d] {
                        max_next = std::cmp::max(max_next, count_starting_at[i + d]);
                    }
                }
            }
            count_starting_at[i] = 1 + max_next;
        } else {
            count_starting_at[i] = 0;
        }

        dp[i] = std::cmp::max(dp[i + 1], count_starting_at[i]);
    }

    dp[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Pattern:
101101?
1??????
??110?1
00???00

String:
1011011";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
