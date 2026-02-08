// Day 22: Passwords
// https://lovemathboy.github.io/day22.html

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day22.txt").ok()?;
    Some((part1(&input).to_string(), part2(&input).to_string()))
}

struct User {
    digit: u32,
    number: u32,
}

fn parse_input(input: &str) -> Vec<User> {
    input
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .map(|block| {
            let mut lines = block.lines();
            let _user_line = lines.next().unwrap();
            let digit_line = lines.next().unwrap();
            let number_line = lines.next().unwrap();

            let digit = digit_line
                .trim()
                .strip_prefix("Favorite Digit: ")
                .unwrap()
                .parse()
                .unwrap();

            let number = number_line
                .trim()
                .strip_prefix("Favorite Number: ")
                .unwrap()
                .parse()
                .unwrap();

            User { digit, number }
        })
        .collect()
}

fn solve_user(user: &User) -> String {
    let m = user.number as usize;
    let fav = user.digit;

    let mut suffixes: Vec<Vec<i32>> = Vec::new();
    // suffixes[0]
    suffixes.push(vec![-1; m]);
    suffixes[0][0] = 0;

    let mut ten_pow = vec![1 % m]; // 10^0 % m

    for len in 1.. {
        // Expand suffixes table
        let mut next_layer = vec![-1; m];
        let prev_layer = &suffixes[len - 1];

        // Compute 10^{len-1} % m
        if len > ten_pow.len() {
            let last = *ten_pow.last().unwrap();
            ten_pow.push((last * 10) % m);
        }
        let p10 = ten_pow[len - 1];

        for (r, &prev_val) in prev_layer.iter().enumerate() {
            if prev_val == -1 {
                continue;
            }
            let c = prev_val;
            for d in 0..10 {
                // Suffix grows to the left: digit d at 10^{len-1}
                // new_rem = (d * 10^{len-1} + r) % m
                let val = (d as usize * p10 + r) % m;
                let nc = c + if d == fav { 1 } else { 0 };
                if nc > next_layer[val] {
                    next_layer[val] = nc;
                }
            }
        }
        suffixes.push(next_layer);

        // Check if solution exists for length `len`
        let req_fav = (len as i32 + 1) / 2;
        let p10_top = ten_pow[len - 1];
        for d1 in 1..10 {
            // Leading digit 1-9
            let term = (d1 as usize * p10_top) % m;
            // We need suffix remainder `needed` such that (term + needed) % m == 0
            // needed = (m - term) % m
            let needed = (m - term) % m;
            let suffix_max_fav = suffixes[len - 1][needed];
            if suffix_max_fav != -1 {
                let total_fav = suffix_max_fav + if d1 == fav { 1 } else { 0 };
                if total_fav >= req_fav {
                    // Solution found! Reconstruct it.
                    return reconstruct(len, m, fav, &suffixes, &ten_pow);
                }
            }
        }
    }
    unreachable!()
}

fn reconstruct(len: usize, m: usize, fav: u32, suffixes: &[Vec<i32>], ten_pow: &[usize]) -> String {
    let mut result = String::new();
    let mut current_rem_sum = 0; // accumulated sum % m of chosen digits
    let mut current_fav_count = 0;
    let req_fav = (len as i32 + 1) / 2;

    for i in (0..len).rev() {
        // i is power: 10^i. Loop len-1 down to 0.
        let is_first = i == (len - 1);
        let start_d = if is_first { 1 } else { 0 };

        for d in start_d..10 {
            let term = (d as usize * ten_pow[i]) % m;
            let next_sum = (current_rem_sum + term) % m;
            let needed_suffix_rem = (m - next_sum) % m;

            let suffix_max_fav = suffixes[i][needed_suffix_rem];
            if suffix_max_fav != -1 {
                let new_fav = current_fav_count + if d == fav { 1 } else { 0 };
                if new_fav + suffix_max_fav >= req_fav {
                    result.push(std::char::from_digit(d, 10).unwrap());
                    current_rem_sum = next_sum;
                    current_fav_count = new_fav;
                    break;
                }
            }
        }
    }
    result
}

fn part1(input: &str) -> usize {
    let users = parse_input(input);
    let mut sum: u64 = 0;
    for user in users {
        let s = solve_user(&user);
        let val: u64 = s.parse().unwrap();
        sum += val;
    }
    sum as usize
}

fn count_valid(user: &User, len: usize) -> usize {
    let m = user.number as usize;
    let fav = user.digit;
    let req_fav = len.div_ceil(2);

    // dp[i][rem][cnt]
    // i: number of digits placed so far
    // rem: current remainder
    // cnt: count of favorite digits
    // We can optimize space: we only need previous layer.

    // However, since we need to handle "no leading zero", the first digit is special (1..=9).
    // Subsequent digits are 0..=9.

    // dp[rem][cnt] -> count
    let max_cnt = len;
    let mut dp = vec![vec![0usize; max_cnt + 1]; m];

    // Initialize for first digit (index 0)
    for d in 1..=9 {
        let r = (d as usize) % m;
        let c = if d == fav { 1 } else { 0 };
        dp[r][c] += 1;
    }

    for _i in 1..len {
        let mut next_dp = vec![vec![0usize; max_cnt + 1]; m];
        for (r, row) in dp.iter().enumerate() {
            for (c, &count) in row.iter().enumerate() {
                if count == 0 {
                    continue;
                }
                // let count = dp[r][c]; // Already have it

                for d in 0..=9 {
                    let next_r = (r * 10 + d as usize) % m;
                    let next_c = c + if d == fav { 1 } else { 0 };
                    if next_c <= max_cnt {
                        next_dp[next_r][next_c] += count;
                    }
                }
            }
        }
        dp = next_dp;
    }

    let mut total = 0;
    for &count in dp[0].iter().skip(req_fav) {
        total += count;
    }
    total
}

fn part2(input: &str) -> usize {
    let users = parse_input(input);
    let mut total = 0;
    for user in users {
        for len in 8..=16 {
            total += count_valid(&user, len);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
User 1:
Favorite Digit: 8
Favorite Number: 2

User 2:
Favorite Digit: 4
Favorite Number: 38

User 3:
Favorite Digit: 5
Favorite Number: 492

User 4:
Favorite Digit: 0
Favorite Number: 3
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 56128);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 591318956547);
    }
}
