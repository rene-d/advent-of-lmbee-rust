// Day 19: Grid Traversal
// https://lovemathboy.github.io/day19.html

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day19.txt").ok()?;
    Some((part1(&input).to_string(), part2(&input).to_string()))
}

fn parse_input(input: &str) -> Vec<Vec<Vec<u8>>> {
    input
        .trim()
        .split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|row| row.bytes().map(|c| c - b'0').collect())
                .collect()
        })
        .collect()
}

fn solve_grid(grid: &[Vec<u8>]) -> u64 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    if cols == 0 {
        return 0;
    }

    let mut dp = vec![vec![0; cols]; rows];

    // Start position
    dp[0][0] = grid[0][0] as u64;

    // First row
    for j in 1..cols {
        dp[0][j] = dp[0][j - 1] + grid[0][j] as u64;
    }

    // First col
    for i in 1..rows {
        dp[i][0] = dp[i - 1][0] + grid[i][0] as u64;
    }

    // The rest
    for i in 1..rows {
        for j in 1..cols {
            dp[i][j] = grid[i][j] as u64 + dp[i - 1][j].max(dp[i][j - 1]);
        }
    }

    dp[rows - 1][cols - 1]
}

fn part1(input: &str) -> u64 {
    let grids = parse_input(input);
    grids.iter().map(|grid| solve_grid(grid)).product()
}

fn solve_2_robots(grid: &[Vec<u8>]) -> u64 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    if cols == 0 {
        return 0;
    }

    // Robot 1: TL to BR (Down, Right)
    // dp1_start[i][j]: max path from (0,0) to (i,j)
    let mut dp1_start = vec![vec![0; cols]; rows];
    dp1_start[0][0] = grid[0][0] as u64;
    for j in 1..cols {
        dp1_start[0][j] = dp1_start[0][j - 1] + grid[0][j] as u64;
    }
    for i in 1..rows {
        dp1_start[i][0] = dp1_start[i - 1][0] + grid[i][0] as u64;
    }
    for i in 1..rows {
        for j in 1..cols {
            dp1_start[i][j] =
                grid[i][j] as u64 + std::cmp::max(dp1_start[i - 1][j], dp1_start[i][j - 1]);
        }
    }

    // dp1_end[i][j]: max path from (i,j) to (R-1, C-1)
    let mut dp1_end = vec![vec![0; cols]; rows];
    dp1_end[rows - 1][cols - 1] = grid[rows - 1][cols - 1] as u64;
    for j in (0..cols - 1).rev() {
        dp1_end[rows - 1][j] = dp1_end[rows - 1][j + 1] + grid[rows - 1][j] as u64;
    }
    for i in (0..rows - 1).rev() {
        dp1_end[i][cols - 1] = dp1_end[i + 1][cols - 1] + grid[i][cols - 1] as u64;
    }
    for i in (0..rows - 1).rev() {
        for j in (0..cols - 1).rev() {
            dp1_end[i][j] = grid[i][j] as u64 + std::cmp::max(dp1_end[i + 1][j], dp1_end[i][j + 1]);
        }
    }

    // Robot 2: BL to TR (Up, Right)
    // dp2_start[i][j]: max path from (R-1, 0) to (i,j)
    let mut dp2_start = vec![vec![0; cols]; rows];
    dp2_start[rows - 1][0] = grid[rows - 1][0] as u64;
    for j in 1..cols {
        dp2_start[rows - 1][j] = dp2_start[rows - 1][j - 1] + grid[rows - 1][j] as u64;
    }
    for i in (0..rows - 1).rev() {
        dp2_start[i][0] = dp2_start[i + 1][0] + grid[i][0] as u64;
    }
    for i in (0..rows - 1).rev() {
        for j in 1..cols {
            dp2_start[i][j] =
                grid[i][j] as u64 + std::cmp::max(dp2_start[i + 1][j], dp2_start[i][j - 1]);
        }
    }

    // dp2_end[i][j]: max path from (i,j) to (0, C-1)
    let mut dp2_end = vec![vec![0; cols]; rows];
    dp2_end[0][cols - 1] = grid[0][cols - 1] as u64;
    for j in (0..cols - 1).rev() {
        dp2_end[0][j] = dp2_end[0][j + 1] + grid[0][j] as u64;
    }
    for i in 1..rows {
        dp2_end[i][cols - 1] = dp2_end[i - 1][cols - 1] + grid[i][cols - 1] as u64;
    }
    for i in 1..rows {
        for j in (0..cols - 1).rev() {
            dp2_end[i][j] = grid[i][j] as u64 + std::cmp::max(dp2_end[i - 1][j], dp2_end[i][j + 1]);
        }
    }

    let mut max_score = 0;

    // Iterate over possible intersection points (not on the border)
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            // Case 1: Robot 1 Vertical (comes from Top, goes Bottom), Robot 2 Horizontal (comes from Left, goes Right)
            // R1: (i-1, j) -> (i, j) -> (i+1, j)
            // R2: (i, j-1) -> (i, j) -> (i, j+1)
            let score1 = dp1_start[i - 1][j] + dp1_end[i + 1][j];
            let score2 = dp2_start[i][j - 1] + dp2_end[i][j + 1];
            // Add intersection twice (once for each robot)
            let total1 = score1 + score2 + 2 * grid[i][j] as u64;
            max_score = std::cmp::max(max_score, total1);

            // Case 2: Robot 1 Horizontal (comes from Left, goes Right), Robot 2 Vertical (comes from Bottom, goes Top)
            // R1: (i, j-1) -> (i, j) -> (i, j+1)
            // R2: (i+1, j) -> (i, j) -> (i-1, j)
            let score3 = dp1_start[i][j - 1] + dp1_end[i][j + 1];
            let score4 = dp2_start[i + 1][j] + dp2_end[i - 1][j];
            let total2 = score3 + score4 + 2 * grid[i][j] as u64;
            max_score = std::cmp::max(max_score, total2);
        }
    }

    max_score
}

fn part2(input: &str) -> u64 {
    let grids = parse_input(input);
    grids.iter().map(|grid| solve_2_robots(grid)).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
19532
36182
93847
85364
17385

123
456
789";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1450);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4650);
    }
}
