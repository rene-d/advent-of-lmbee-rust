mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

const SOLUTIONS: [(u32, &str, &str); 13] = [
    (13, "87485764037410", "1019690398768"),
    (14, "75292", "824149222686720"),
    (15, "82548913169553421", "181256043161563369"),
    (16, "1316", "69"),
    (17, "492", "492"),
    (18, "747", "3787"),
    (19, "5339472513000210", "155109234149896320"),
    (20, "10000000000000016", "420156170059586436"),
    (21, "12562624846200429", "3470"),
    (22, "125269710", "5193340566916"),
    (23, "2177210444409", "92672493957120"),
    (24, "13948707050", "8783628093237420033"),
    (25, "875", "492"),
];

fn main() {
    let mut args = std::env::args();
    let _ = args.next();

    if let Some(day) = args.next() {
        solve(day);
    } else {
        for day in 13..=25 {
            solve(day.to_string());
            println!();
        }
    }
}

fn solve(day: String) {
    println!("🎁 Day {}: ", day);

    let answer = match day.as_str() {
        "13" => crate::day13::solve(),
        "14" => crate::day14::solve(),
        "15" => crate::day15::solve(),
        "16" => crate::day16::solve(),
        "17" => crate::day17::solve(),
        "18" => crate::day18::solve(),
        "19" => crate::day19::solve(),
        "20" => crate::day20::solve(),
        "21" => crate::day21::solve(),
        "25" => crate::day25::solve(),
        "22" => crate::day22::solve(),
        "23" => crate::day23::solve(),
        "24" => crate::day24::solve(),
        _ => None,
    };

    if let Some((p1, p2)) = answer {
        if let Some((_, expected_p1, expected_p2)) =
            SOLUTIONS.iter().find(|(d, _, _)| d.to_string() == day)
        {
            if p1 == *expected_p1 {
                println!("  Part 1: {p1} ✅");
            } else {
                println!("  Part 1: {p1} ❌ (expected {})", expected_p1);
            }
            if p2 == *expected_p2 {
                println!("  Part 2: {p2} ✅");
            } else {
                println!("  Part 2: {p2} ❌ (expected {})", expected_p2);
            }
        }
    } else {
        println!("unknown day '{day}' or input not found");
    }
}
