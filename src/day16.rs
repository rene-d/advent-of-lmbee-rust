// Day 16: Remedial Lessons
// https://lovemathboy.github.io/day16.html

pub fn solve() -> Option<(String, String)> {
    let input = std::fs::read_to_string("inputs/day16.txt").ok()?;
    Some((part1(&input).to_string(), part2(&input).to_string()))
}

#[derive(Debug, Clone, Copy)]
struct Lesson {
    start: u64,
    end: u64,
}

fn parse_line(line: &str) -> Option<Lesson> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 13 {
        return None;
    }

    // "Lesson #1: Starts at t = 471925571 and ends at t = 481914514"
    // 0: Lesson
    // 1: #1:
    // 2: Starts
    // 3: at
    // 4: t
    // 5: =
    // 6: 471925571
    // 7: and
    // 8: ends
    // 9: at
    // 10: t
    // 11: =
    // 12: 481914514

    let start = parts[6].parse().ok()?;
    let end = parts[12].parse().ok()?;

    Some(Lesson { start, end })
}

fn parse_input(input: &str) -> Vec<Lesson> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(parse_line)
        .collect()
}

fn part1(input: &str) -> u64 {
    let mut lessons = parse_input(input);
    // Sort by end time
    lessons.sort_by_key(|l| l.end);

    let mut count = 0;
    let mut last_end = 0;

    for lesson in lessons {
        if lesson.start >= last_end {
            count += 1;
            last_end = lesson.end;
        }
    }

    count
}

fn part2(input: &str) -> u64 {
    let lessons = parse_input(input);
    let mut events = Vec::new();

    for lesson in lessons {
        // (time, type): type -1 for end, +1 for start
        // We want to process END before START at the same time to allow [a, b] and [b, c] on same track.
        // So we use -1 for end and +1 for start and sort naturally.
        events.push((lesson.start, 1));
        events.push((lesson.end, -1));
    }

    // Sort by time, then by type.
    // -1 comes before 1, so ends are processed before starts at the same timestamp.
    events.sort();

    let mut max_overlap = 0;
    let mut current_overlap = 0;

    for (_, type_) in events {
        current_overlap += type_;
        if current_overlap > max_overlap {
            max_overlap = current_overlap;
        }
    }

    max_overlap as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Lesson #1: Starts at t = 0 and ends at t = 20
Lesson #2: Starts at t = 50 and ends at t = 150
Lesson #3: Starts at t = 180 and ends at t = 200
Lesson #4: Starts at t = 190 and ends at t = 240
Lesson #5: Starts at t = 10 and ends at t = 40
Lesson #6: Starts at t = 30 and ends at t = 170
Lesson #7: Starts at t = 160 and ends at t = 190";

    #[test]
    fn test_parse() {
        let lessons = parse_input(TEST_INPUT);
        assert_eq!(lessons.len(), 7);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}
