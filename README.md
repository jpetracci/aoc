# Advent of Code attempts

Advent of Code solutions/attempts. I am learning Rust as I go.

Selected Rust and Python exercises for [Advent of Code](https://adventofcode.com/),
not a complete archive of solved years. A directory or source file indicates an
attempt, not that both parts are complete or verified.

## Coverage in this snapshot

| Year | Attempts present | Language/layout |
| --- | --- | --- |
| 2015 | Days 1–3 | Python for days 1–2; a Rust crate for day 3 |
| 2019 | Days 1–2 (`1a.py`, `1b.py`, `2a.py`, `2b.py`) | Standalone Python scripts |
| 2020 | Days 1–5 | Separate Rust crate per day |
| 2021 | Days 1–3 | Separate Rust crate per day |
| 2022 | Days 1–6 | Separate Rust crate per day |

The Rust crates are not a root Cargo workspace. Manifests inspected from the
2015/2020 exercises use edition 2018, while the inspected 2021/2022 manifests
use edition 2021. Consult each day's `Cargo.toml` and `Cargo.lock` where present
for dependencies; there is no exact Rust compiler pin for the collection.
Python scripts use Python 3 syntax. No collection-wide runtime compatibility or
completion claim is made.

## Inputs stay private

Obtain your own user-specific inputs from Advent of Code; do not publish them
or copy someone else's inputs as shared fixtures. See the
[Advent of Code FAQ](https://adventofcode.com/about#faq_copying). Some input files
are already tracked in this historical tree, while others are missing. This
README neither reproduces nor validates their contents. Keep your own downloads
local; do not stage replacements of tracked files, and exclude new private
input paths locally (for example, via `.git/info/exclude`).

Check each program's file-reading path before running: paths are relative to
its **working directory**, not automatically to the source file. In particular:

- 2015 Python days 1–2 open `2015/day1/input.txt` and `2015/day2/input.txt` from
  the repository root. Rust day 3 opens `input.txt` from its crate directory.
- 2019 scripts open `1a.txt`, `1b.txt`, or `2a.txt` from the working directory;
  `2b.py` also uses `2a.txt`. These files are missing from the tree.
- Rust examples include `2020/day4/day4_input.txt`,
  `2021/day1/day1input.txt`, and `2022/day5/input.txt` when launched from their
  respective crate directories. Other days have their own names; read the
  corresponding source rather than assuming one common input filename.
- 2022 days 3–6 expect local `input.txt` files that are not in the tree.

## Run examples

From the repository root, with your own inputs placed at the expected paths:

```sh
# 2015 Python examples use root-relative paths:
python3 2015/day1/day1.py
python3 2015/day2/day2.py

# 2015 Rust example:
(cd 2015/day3 && cargo run)

# 2019 scripts use files relative to this directory:
(cd 2019 && python3 1a.py)
(cd 2019 && python3 2b.py)

# Run other Rust days from their individual crate directories:
(cd 2020/day4 && cargo run)
(cd 2021/day1 && cargo run)
(cd 2022/day5 && cargo run)
```

Substitute another day directory/script as appropriate. `cargo test` can be run
inside an individual crate, but test presence and coverage vary; it is not a
collection-wide answer check. No solution execution, test pass, or puzzle-answer
verification is claimed by these instructions.
