# Advent of Lmbee 2025 in Rust

Solutions for the [Advent of Lmbee 2025](https://lovemathboy.github.io/) event, implemented in Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

## How to Run

To run a solution for a specific day, use the following command:

```bash
cargo run --release -q -- <day>
```

For example, to run Day 14:

```bash
cargo run --release -q -- 14
```

If no day is specified, all days will be run.

The program will automatically verify the output against the known correct answers stored in `src/main.rs`.

## Project Structure

- `src/`: Contains the Rust source code for each day's solution.
- `inputs/`: Contains the input files for each day.
- `lovemathboy.github.io/`: Contains the HTML files of the event website (used for extracting solutions) (submodule).

## Solutions Extraction

The following Python script parses the HTML files from the event website to extract the expected answers for Part 1 and Part 2. These answers are used to generate the `SOLUTIONS` constant in `src/main.rs` for verification.

```python
from pathlib import Path
import re

solutions: dict[int, dict[str, str]] = {}

for file in Path("lovemathboy.github.io").glob("day*.html"):
    day = int(file.stem.replace("day", ""))
    solutions[day] = {}
    for line in file.read_text().splitlines():
        if m := re.search(r"part1: '(.+)',", line):
            solutions[day]["part1"] = m.group(1)

        if m := re.search(r"part2: '(.+)'", line):
            solutions[day]["part2"] = m.group(1)

print("const SOLUTIONS: [(u32, &str, &str); " + str(len(solutions)) + "] = [")
for day in sorted(solutions.keys()):
    p1 = solutions[day].get("part1", "")
    p2 = solutions[day].get("part2", "")
    print(f'    ({day}, "{p1}", "{p2}"),')
print("];")
```
