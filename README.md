# Advent of Code 2025

Meine Lösungen für [Advent of Code](https://adventofcode.com/) in Rust.

[![Rust Tests](https://github.com/felixrieg/AoC2025/workflows/Rust%20Tests/badge.svg)](https://github.com/felixrieg/AoC2025/actions)

## Verwendung

### Eine Challenge ausführen

```bash
cargo run <day>
cargo run 1    # Tag 1 ausführen
cargo run 15   # Tag 15 ausführen
cargo run      # Alle Tage ausführen
```

### Tests ausführen

```bash
cargo test              # Alle Tests
cargo test utils::      # Nur Utils-Tests
cargo test types::      # Nur Range-Tests
cargo test -- --nocapture  # Mit Output
```

## Projektstruktur

```
.
├── src/
│   ├── main.rs          # Entry Point mit Registry Pattern
│   ├── utils.rs         # Input-Handling Funktionen
│   ├── types/           # Custom Types (Range, etc.)
│   └── days/            # Lösungen für jeden Tag
├── inputs/              # Puzzle-Inputs
│   ├── day01.txt
│   ├── day02.txt
│   └── ...
└── .github/workflows/   # CI/CD Pipeline
```

### Neuen Tag hinzufügen

1. Erstelle `src/days/dayXX.rs` mit `pub fn solve()` Funktion
2. Registriere in `src/days/mod.rs`: `pub mod dayXX;`
3. Füge zu `AVAILABLE_DAYS` in `src/main.rs` hinzu: `(XX, days::dayXX::solve),`

## Tests

- **Utils Tests**: 18 Tests für Input-Handling Funktionen
- **Range Tests**: 30+ Tests für Range Struct und Iterator
- Läuft automatisch bei jedem Push mit GitHub Actions

## Utils Module

| Funktion                         | Beschreibung                 |
| -------------------------------- | ---------------------------- |
| `read_input(day)`                | Liest komplette Input-Datei  |
| `read_lines(day)`                | Liest Input zeilenweise      |
| `read_numbers(day)`              | Parst alle Zahlen aus Input  |
| `read_input_and_split(day, sep)` | Splittet nach Separator      |
| `split_on_empty_lines(input)`    | Gruppiert nach leeren Zeilen |

## Range Struct

Iterator für Zahlenbereiche mit Methoden wie `contains()`, `overlaps_with()`, `merge_into()`. Kann in for-Schleifen, mit filter/map/fold verwendet werden.

```rust
let r = Range::from_string("1-5".to_string());
for i in r {
    println!("{}", i);  // 1, 2, 3, 4, 5
}
```
