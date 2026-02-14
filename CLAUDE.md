# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Language

常に日本語で会話する。

## Build & Run Commands

```bash
# ビルド＆実行
cargo run

# リリースビルド
cargo run --release

# テスト実行（game-pluginに全46テストが集中）
cargo test -p game-plugin

# 単一テスト実行
cargo test -p game-plugin <テスト名>

# WASM ビルド
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
  --out-dir ./out/ --out-name "bevy-life-game" \
  ./target/wasm32-unknown-unknown/release/bevy-life-game.wasm
```

## Development Philosophy

- TDD（テスト駆動開発）で進める：テスト作成 → 失敗確認 → 実装 → パス確認
- 実装中はテストを変更せず、コードを修正し続ける

## Architecture

**4クレートのワークスペース構成：**

- **root binary** (`src/main.rs`) — Bevyプラグイン統合。DefaultPlugins + 3プラグイン登録
- **common** — 共有基盤。GameState（Title→Game↔Menu）、GameAssets、定数（`consts.rs`に全集約）
- **game-plugin** — ライフゲーム本体。シミュレーション、入力、描画、UI、音声
- **title-plugin** — タイトル画面（Start→GameState::Game）
- **menu-plugin** — メニュー画面（Back/Quit、Escで遷移）

**状態管理：**
- `GameState`（Title/Game/Menu）でプラグイン単位のシステム有効化
- `SimulationState`（Paused/Simulating）はgame-plugin内部のみ

**イベントシステム（Bevy 0.18）：**
- `Message` deriveマクロ + `MessageWriter`/`MessageReader`（旧EventWriter/EventReaderではない）

**描画アーキテクチャ：**
- 100x100グリッドを単一Image テクスチャ + Spriteで描画（個別メッシュエンティティではない）
- セルデータは`Vec<bool>`フラット配列
- デュアルビューポート：サイドメニュー(20%) + ワールド(80%)

## Bevy 0.18 API注意点

- `with_children`のクロージャ引数は`ChildSpawnerCommands`（`ChildSpawner`ではない）
- `ChildSpawnerCommands::spawn()`は`EntityCommands`を返す（`EntityWorldMut`ではない）
- Timer: `is_finished()`を使う（`finished()`は非公開）
- `RenderAssetUsages`のインポート: `bevy::asset::RenderAssetUsages`
- クロージャ内の`&str`は`.to_string()`が必要
- ピクセルパーフェクト: `ImageSampler::Descriptor(ImageSamplerDescriptor::nearest())`

## Testing

テストはgame-plugin内に集中（46テスト）:
- `resources/world.rs` — Worldロジック（22テスト）
- `resources/simulation.rs` — 純粋シミュレーション関数（9テスト）
- `systems/action.rs` — 座標変換（8テスト）
- `systems/input.rs` — ズーム・パン（7テスト）

## Key Constants

全チューニング定数は `common/src/consts.rs` に集約。シミュレーション速度、カメラ設定、ウィンドウサイズ等。
