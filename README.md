# Conway's Life Game with Bevy

[Bevy](https://bevyengine.org/) (0.18) で実装した [Conway's Game of Life](https://ja.wikipedia.org/wiki/%E3%83%A9%E3%82%A4%E3%83%95%E3%82%B2%E3%83%BC%E3%83%A0) です。

## アーキテクチャ

4クレートのワークスペース構成：

| クレート | 役割 |
|---|---|
| **root** (`src/main.rs`) | Bevyプラグイン統合。DefaultPlugins + 3プラグイン登録 |
| **common** | 共有基盤。GameState、GameAssets、定数（`consts.rs`に全集約） |
| **game-plugin** | ライフゲーム本体。シミュレーション、入力、描画、UI、音声 |
| **title-plugin** | タイトル画面 |
| **menu-plugin** | メニュー画面（Back/Quit、Escで遷移） |

### 状態管理

- `GameState`（Title / Game / Menu）でプラグイン単位のシステム有効化
- `SimulationState`（Paused / Simulating）は game-plugin 内部のみ

### 描画

- 100x100 グリッドを単一 Image テクスチャ + Sprite で描画
- セルデータは `Vec<bool>` フラット配列
- デュアルビューポート：ワールド（90%）+ ボトムパネル（10%）
- ウィンドウリサイズ対応

## ビルド & 実行

[just](https://github.com/casey/just) コマンドランナーを利用できます。

```sh
# 開発実行
just run        # or: cargo run

# リリースビルド実行
just run-release  # or: cargo run --release

# テスト実行（64テスト）
just test       # or: cargo test -p game-plugin -p common

# フォーマット
just fmt

# Lintチェック
just clippy

# CIパイプライン再現
just ci
```

## WASM ビルド

### Trunk（開発サーバー）

```sh
just dev  # trunk serve
```

### 手動ビルド

```sh
just wasm
```

以下のコマンドが実行されます：

```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ --out-name "bevy-life-game" \
    ./target/wasm32-unknown-unknown/release/bevy-life-game.wasm
```

## Webページへの埋込方法

生成した js/wasm ファイルをプロジェクトの任意の場所に配置し、以下のような HTML ファイルを作成して iframe で埋め込みます。

```html
<!doctype html>
<html lang="en">
  <body style="margin: 0px;">
    <script type="module">
      import init from '<path to js file>/bevy-life-game.js';
      try {
        init();
      } catch (e) {
        console.error(e);
      }
    </script>
  </body>
</html>
```

```html
<iframe src="<path to html file>/index.html" width="1280" height="720"></iframe>
```

### itch.io 用の zip ファイルを作成する方法

以下の内容が入った `index.html` を作成します。

```html
<!doctype html>
<html lang="en">
  <body style="margin: 0px">
    <script type="module">
      import init from "./bevy-life-game.js";
      try {
        init();
      } catch (e) {
        console.error(e);
      }
    </script>
  </body>
</html>
```

assets, js, wasm, index.html をまとめて zip 化します。

```txt
.
├── assets
│   ├── audios
│   │   └── appear-online.ogg
│   └── fonts
│       ├── NotoSansJP-Bold.ttf
│       └── NotoSansJP-Regular.ttf
├── bevy-life-game.js
├── bevy-life-game_bg.wasm
└── index.html
```

> **Note:** `Trunk.build.toml` で `public_url = "./"` を設定済みのため、itch.io でのアセット読み込みに対応しています。

## CI/CD

GitHub Actions で以下のワークフローが設定されています：

- **ci.yml** — フォーマット確認、clippy、テスト、ビルド
- **bump.yml** — cocogitto による自動バージョンバンプ
- **release.yml** — itch.io へのリリース

## テスト

合計 64 テスト（game-plugin: 57、common: 7）：

- `game-plugin/src/resources/world.rs` — World ロジック
- `game-plugin/src/resources/simulation.rs` — 純粋シミュレーション関数
- `game-plugin/src/systems/coordinate.rs` — 座標変換
- `game-plugin/src/systems/input.rs` — ズーム・パン
- `common/src/consts.rs` — ビューポート計算
