# デフォルトレシピ: 開発実行
default: run

# 開発実行
run:
    cargo run

# リリースビルド実行
run-release:
    cargo run --release

# 全テスト実行
test:
    cargo test -p game-plugin

# 単一テスト実行
test-one NAME:
    cargo test -p game-plugin {{NAME}}

# フォーマット自動修正
fmt:
    cargo fmt --all

# フォーマット確認
fmt-check:
    cargo fmt --all -- --check

# Lintチェック
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# CIパイプライン再現
ci: fmt-check clippy test build

# リリースビルド
build:
    cargo build --release --verbose

# WASMビルド
wasm:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --target web \
        --out-dir ./out/ --out-name "bevy-life-game" \
        ./target/wasm32-unknown-unknown/release/bevy-life-game.wasm

# WASM開発サーバー起動
dev:
    trunk serve

# ビルド成果物削除
clean:
    cargo clean
