# Conway's Life Game with Bevy

## ビルド方法

```sh
cargo run --release
```

## Webページへの埋込方法

### rustプロジェクトをリリースビルド

```sh
cargo build --release --target wasm32-unknown-unknown
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

### itch.io用のzipファイルを作成する方法

以下の内容が入ったindex.htmlを作成する

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
    <iframe id="test" src="./index.html" width="1000" height="800"></iframe>
  </body>
</html>
```

assets, js, wasm, index.htmlをまとめてzip化する

```txt
.
├── assets
│   ├── audios
│   │   └── appear-online.ogg
│   └── fonts
│       ├── NotoSansJP-Bold.ttf
│       └── NotoSansJP-Regular.ttf
├── bevy-life-game.js
├── bevy-life-game_bg.wasm
├── index.html
└── アーカイブ.zip
```

DefaultPlugins.setで以下を追加しないとassetファイルが読み込まれない
https://github.com/bevyengine/bevy/issues/10157#issuecomment-2308481813
