# Suggested Commands

## Build & Run
```bash
cargo run                  # Debug build & run
cargo run --release        # Release build & run
cargo build                # Build only
```

## Testing
```bash
cargo test -p game-plugin          # Run all 46 tests
cargo test -p game-plugin <name>   # Run single test
```

## WASM Build
```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
  --out-dir ./out/ --out-name "bevy-life-game" \
  ./target/wasm32-unknown-unknown/release/bevy-life-game.wasm
```

## Formatting & Linting
```bash
cargo fmt          # Format code
cargo clippy       # Lint
```

## System Utilities (macOS/Darwin)
- `git` — Version control
- `ls`, `find`, `grep` — File operations (same as Linux on macOS)
