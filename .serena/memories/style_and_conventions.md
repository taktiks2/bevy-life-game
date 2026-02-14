# Code Style & Conventions

## Language
- All communication in Japanese (日本語)
- Code comments/docs may be in Japanese

## Rust Conventions
- Standard Rust naming: snake_case for functions/variables, PascalCase for types
- Rust edition 2024
- Workspace dependencies centralized in root `Cargo.toml`

## Development Philosophy
- **TDD (Test-Driven Development)**: Write tests first → confirm failure → implement → confirm pass
- During implementation, do NOT modify tests — only modify code
- Tests concentrated in `game-plugin` crate (46 tests)

## Bevy 0.18 Specifics
- `with_children` closure param: `ChildSpawnerCommands` (NOT `ChildSpawner`)
- `ChildSpawnerCommands::spawn()` returns `EntityCommands` (NOT `EntityWorldMut`)
- Events: `Message` derive + `MessageWriter`/`MessageReader`
- Timer: use `is_finished()` (not `finished()`)
- `RenderAssetUsages`: import from `bevy::asset::RenderAssetUsages`
- String in closures: need `.to_string()` for `&str`
- Pixel-perfect: `ImageSampler::Descriptor(ImageSamplerDescriptor::nearest())`

## Architecture Patterns
- Constants centralized in `common/src/consts.rs`
- Single texture rendering (not per-entity meshes)
- Flat `Vec<bool>` for cell data
- Coordinate calculation for click handling (not per-cell observers)
