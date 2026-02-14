# Project Overview

## Purpose
Conway's Game of Life implementation using Bevy game engine (Rust).

## Tech Stack
- **Language**: Rust (edition 2024)
- **Game Engine**: Bevy 0.18
- **Debug**: bevy-inspector-egui 0.36
- **Platform**: macOS (Darwin), also supports WASM

## Workspace Structure
4-crate workspace:
- **root** (`src/main.rs`) — Entry point, registers DefaultPlugins + 3 plugins
- **common** — Shared: GameState enum (Title/Game/Menu), GameAssets, constants (`consts.rs`)
- **game-plugin** — Core: simulation, input, rendering, UI, audio
- **title-plugin** — Title screen
- **menu-plugin** — Menu screen (pause)

## Architecture
- Grid: 100x100, rendered as single Image texture + Sprite (not individual entities)
- Cell data: `Vec<bool>` flat array with `World` struct
- Dual viewport: side menu (20%) + world (80%)
- State: `GameState` (Title/Game/Menu) + `SimulationState` (Paused/Simulating)
- Events: Bevy 0.18 `Message` derive macro + `MessageWriter`/`MessageReader`
- All constants centralized in `common/src/consts.rs`
