// Day 13: Legume Plantation
// https://lovemathboy.github.io/day13.html

use std::mem;

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day13.txt").ok()?;

    Some((part1(&input).to_string(), part2(&input).to_string()))
}

fn part1(input: &str) -> u64 {
    let mut even_heighs = 0;
    let mut odd_heighs = 0;

    let mut even_plants = 0;
    let mut odd_plants = 0;

    for line in input.lines() {
        let (a, b) = line.split_once(' ').unwrap();

        if a == "plant" {
            let height: u64 = b.parse().unwrap();
            if height.is_multiple_of(2) {
                even_heighs += height;
                even_plants += 1;
            } else {
                odd_heighs += height;
                odd_plants += 1;
            }
        } else if a == "spray" {
            match b {
                "all" => {
                    even_heighs += even_plants;
                    odd_heighs += odd_plants;

                    mem::swap(&mut odd_heighs, &mut even_heighs);
                    mem::swap(&mut odd_plants, &mut even_plants);
                }
                "even" => {
                    odd_heighs += even_heighs + even_plants;
                    odd_plants += even_plants;
                    even_heighs = 0;
                    even_plants = 0
                }
                "odd" => {
                    even_heighs += odd_heighs + odd_plants;
                    even_plants += odd_plants;
                    odd_heighs = 0;
                    odd_plants = 0;
                }
                _ => {
                    panic!("bad line {line}");
                }
            }
        }
    }

    even_heighs + odd_heighs
}

fn part2(input: &str) -> u64 {
    let mut evens = Vec::new();
    let mut odds = Vec::new();

    for line in input.lines() {
        let (a, b) = line.split_once(' ').unwrap();

        if a == "plant" {
            let height: u64 = b.parse().unwrap();

            if height.is_multiple_of(2) {
                evens.push(height);
            } else {
                odds.push(height);
            }
        } else if a == "spray" {
            let mut new_evens = Vec::new();
            let mut new_odds = Vec::new();

            if b == "all" || b == "even" {
                let mut i = 0;
                while i < evens.len() {
                    evens[i] /= 2;
                    if evens[i] == 0 {
                        // height is 0, remove it
                        evens.swap_remove(i);
                    } else if evens[i] % 2 != 0 {
                        // height becomes odd, move it to odds
                        new_odds.push(evens.swap_remove(i));
                    } else {
                        // height remains even, keep it
                        i += 1;
                    }
                }
            }

            if b == "all" || b == "odd" {
                let mut i = 0;
                while i < odds.len() {
                    odds[i] /= 2;
                    if odds[i] == 0 {
                        odds.swap_remove(i);
                    } else if odds[i] % 2 == 0 {
                        new_evens.push(odds.swap_remove(i));
                    } else {
                        i += 1;
                    }
                }
            }

            evens.extend(new_evens);
            odds.extend(new_odds);
        }
    }

    evens.iter().sum::<u64>() + odds.iter().sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
plant 5
spray even
spray odd
plant 9
spray all
plant 4
spray even
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 23);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 5);
    }
}
