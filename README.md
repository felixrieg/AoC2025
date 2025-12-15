# Advent of Code 2025

My solutions for [Advent of Code](https://adventofcode.com/) in Rust.

[![Rust Tests](https://github.com/felixrieg/AoC2025/workflows/Rust%20Tests/badge.svg)](https://github.com/felixrieg/AoC2025/actions)

## Usage

### Run a single challenge

```bash
cargo run <day>
cargo run 1    # Run day 1
cargo run 15   # Run day 15
cargo run      # Run all days
```

### Run tests

```bash
cargo test              # All tests
cargo test utils::      # Only utils tests
cargo test types::      # Only types tests
cargo test -- --nocapture  # With output
```

## Project structure

```
.
├── src/
│   ├── main.rs          # Entry point with registry pattern
│   ├── utils.rs         # Input handling functions
│   ├── types/           # Custom types (Range, etc.)
│   └── days/            # Solutions for each day
├── inputs/              # Puzzle inputs
│   ├── day01.txt
│   ├── day02.txt
│   └── ...
└── .github/workflows/   # CI/CD pipeline
```

### Add a new day

1. Create `src/days/dayXX.rs` with `pub fn solve()` function
2. Register in `src/days/mod.rs`: `pub mod dayXX;`
3. Add to `AVAILABLE_DAYS` in `src/main.rs`: `(XX, days::dayXX::solve),`

## Tests

- **Utils Tests**: 18 tests for input handling functions
- **Types Tests**: 30+ tests for Range struct and iterators
- **UnionFind Tests**: 9 tests for Union-Find data structure
- Runs automatically on every push with GitHub Actions

## Utils module

| Function                         | Description                |
| -------------------------------- | -------------------------- |
| `read_input(day)`                | Read complete input file   |
| `read_lines(day)`                | Read input line by line    |
| `read_numbers(day)`              | Parse all numbers from input |
| `read_input_and_split(day, sep)` | Split by separator         |
| `split_on_empty_lines(input)`    | Group by empty lines       |

## Range struct

Iterator for number ranges with methods like `contains()`, `overlaps_with()`, `merge_into()`. Can be used in for-loops, with filter/map/fold.

```rust
let r = Range::from_string("1-5".to_string());
for i in r {
    println!("{}", i);  // 1, 2, 3, 4, 5
}
```

## UnionFind struct

Efficient disjoint set union data structure with path compression. Useful for clustering problems.

```rust
let mut uf = UnionFind::new(5);
uf.union(0, 1);
uf.union(1, 2);
let clusters = uf.cluster_sizes();
```
