# Conway's Life Game with Bevy

## ビルド方法

```sh
cargo run --release
```

## Webページへの埋込方法

### rustプロジェクトをリリースビルド

```sh
cargo build --release --target wasm32-unknown-unknown                                              44.8s
```

### ビルドファイルからwasm-bindgenを使ってjs/wasmファイルを生成

```sh
wasm-bindgen --no-typescript --target web \
      --out-dir ./out/ \
      --out-name "bevy-life-game" \
      ./target/wasm32-unknown-unknown/release/bevy-life-game.wasm
```

### iframeに埋め込み

- 生成したjs/wasmファイルをプロジェクトの任意の場所に配置
- 以下のようなHTMLファイルを作成し、iframeで埋め込む

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

参考サイト
https://bevy-cheatbook.github.io/platforms/wasm/webpage.html
