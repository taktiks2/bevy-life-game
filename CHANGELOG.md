# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.2.0](https://github.com/taktiks2/bevy-life-game/compare/1fc83a37a7705bc60f71deb5c664ed23b2a9ef26..0.2.0) - 2026-02-15
#### Features
- セル間のグリッドライン表示機能を追加 - ([04c692c](https://github.com/taktiks2/bevy-life-game/commit/04c692caef72858b16c35f94bd9f3023f59d90d5)) - taktiks2
- ワールドのサイズ調整 - ([13b576b](https://github.com/taktiks2/bevy-life-game/commit/13b576b745146a3d337f16c0fd7bb002d084940b)) - taktiks2
#### Bug Fixes
- グリッド非表示時にセル色で塗りつぶし＆ズームアウト時の線省略を防止 - ([149e6ab](https://github.com/taktiks2/bevy-life-game/commit/149e6ab1b3dbbca35c469b7bc70f894641745813)) - taktiks2
- ボタンテキストのホバーによる二重イベント発火を修正 - ([266a5cc](https://github.com/taktiks2/bevy-life-game/commit/266a5cccd73e088e6e173e538145278a10aba5a4)) - taktiks2
#### Miscellaneous Chores
- コメントの追加 - ([1fc83a3](https://github.com/taktiks2/bevy-life-game/commit/1fc83a37a7705bc60f71deb5c664ed23b2a9ef26)) - taktiks2

- - -

## [0.1.0](https://github.com/taktiks2/bevy-life-game/compare/88fc59a3817a21e5bc8275b4777952512145f0c4..0.1.0) - 2026-02-15
#### Features
- 初期ウィンドウサイズの変更 - ([a309dd8](https://github.com/taktiks2/bevy-life-game/commit/a309dd8501343d102d5aa798f5aac2b8ca4ede71)) - taktiks2
- ウィンドウリサイズ対応とビューポートの動的計算 - ([58cbc96](https://github.com/taktiks2/bevy-life-game/commit/58cbc965436aaaaf16a8d65aadc36c4183bd80fb)) - taktiks2
- ボトムパネルのサイズ変更 - ([b2e922a](https://github.com/taktiks2/bevy-life-game/commit/b2e922a4805c8dfda246d126f637a17fdf4b1064)) - taktiks2
- タイトルの変更 - ([ac3661a](https://github.com/taktiks2/bevy-life-game/commit/ac3661a74a6551fedb9e72e0d4b83b3f1f2d7bd1)) - taktiks2
- サイドパネルをボトムパネルに変更し、ボタンスタイルを強化 - ([6b0bb68](https://github.com/taktiks2/bevy-life-game/commit/6b0bb6839ffa52b85950383b79c28db2ce925dba)) - taktiks2
- ダークテーマ+ネオングリーンのモダンUIデザインに全面リニューアル - ([6570a1f](https://github.com/taktiks2/bevy-life-game/commit/6570a1f937d708098517b0175c6b22c9a70d9270)) - taktiks2
- itch.ioリリース用のGitHub ActionsワークフローとWASMビルド設定の追加 - ([5fb0638](https://github.com/taktiks2/bevy-life-game/commit/5fb0638c9a4a15ce612b95c336f76373c2551f8b)) - taktiks2
- タイトルの変更 - ([4766b29](https://github.com/taktiks2/bevy-life-game/commit/4766b29e91ad270b54f28971f91c53325f03241d)) - taktiks2
- 0.18へのアップグレード - ([0f7baf3](https://github.com/taktiks2/bevy-life-game/commit/0f7baf3d980a698e21fd7770d08745265fa689cf)) - taktiks2
#### Bug Fixes
- (**ci**) ブランチ保護ルールをバイパスするためPATを使用 - ([803c99d](https://github.com/taktiks2/bevy-life-game/commit/803c99d2c9d34d9113b61682e23611aa0125f7cd)) - taktiks2
- (**ci**) bump.ymlの一時ファイルをリポジトリ外に作成しcogのpre-bump checks失敗を修正 - ([c7e1e7f](https://github.com/taktiks2/bevy-life-game/commit/c7e1e7f1cd195b16d5ef40a166ae23b2acebac0d)) - taktiks2
- (**ci**) CIワークフローにBevy用Linux依存パッケージのインストールを追加 - ([c0e86ac](https://github.com/taktiks2/bevy-life-game/commit/c0e86ac383cb7a898ef3aa06cd3c27a07009384d)) - taktiks2
- ボトムパネルの高さをウィンドウサイズに依存しない固定値に変更 - ([bf8562d](https://github.com/taktiks2/bevy-life-game/commit/bf8562d49de6cac27d80ac52a3718c4d7995f8a6)) - taktiks2
- ボトムパネル上でのグリッド操作を無効化 - ([b012f0c](https://github.com/taktiks2/bevy-life-game/commit/b012f0cf36d915a7b8ef4c48d29ecb45b771eff6)) - taktiks2
- WASM環境でアセットが読み込まれるようcopy-dirディレクティブを追加 - ([b960421](https://github.com/taktiks2/bevy-life-game/commit/b960421044bb4cfb20eda94438d31ee7998f59c4)) - taktiks2
- butlerインストールをsetup-butler Actionに置き換え - ([1bd95de](https://github.com/taktiks2/bevy-life-game/commit/1bd95de819fa0d03ba290cd81233e35e18a82eb5)) - taktiks2
- clippy警告の解消（matches!マクロ適用・未使用メソッドのテスト専用化） - ([ae1228a](https://github.com/taktiks2/bevy-life-game/commit/ae1228a653c9f60f204ba1baa02e7ef7b2722721)) - taktiks2
#### Continuous Integration
- cogによる自動バージョンバンプとリリースワークフローの分離 - ([b25a121](https://github.com/taktiks2/bevy-life-game/commit/b25a12129c7cc94457038c827ee05b6e56b61b69)) - taktiks2
#### Refactoring
- セルサイズ計算の共通化とtitle/menuプラグインのUI共通化 - ([fa641ca](https://github.com/taktiks2/bevy-life-game/commit/fa641ca1d36b9905c1563a079749965d0e90d3eb)) - taktiks2
- action.rsの分割とアーキテクチャ整理 - ([b5c5735](https://github.com/taktiks2/bevy-life-game/commit/b5c57358a4240af60e9213d2ad356b89f0b35bf8)) - taktiks2
- mod.rsの削除 - ([1e83b77](https://github.com/taktiks2/bevy-life-game/commit/1e83b7711bd4581fe31e571e9b7904fbb8b37534)) - taktiks2
- セル描画とインタラクションのパフォーマンス大幅改善 - ([3bfc603](https://github.com/taktiks2/bevy-life-game/commit/3bfc603d4d575ae7f185416a20ec033d635e29d5)) - taktiks2
- パフォーマンス改善とテスタビリティ向上のためのアーキテクチャ改善 - ([f5fb87a](https://github.com/taktiks2/bevy-life-game/commit/f5fb87a6895e56a7dd58bd654e758ffe3129b964)) - taktiks2
- 保守性向上のための大規模リファクタリング - ([4e1d5f9](https://github.com/taktiks2/bevy-life-game/commit/4e1d5f9df33fda2178f8bc0408a780dc9a7cc698)) - taktiks2
- workspace依存の一元化とedition 2024への更新 - ([461ea56](https://github.com/taktiks2/bevy-life-game/commit/461ea5676c2e73684f5dfd46cc4c401446a12752)) - taktiks2
#### Miscellaneous Chores
- .gitignoreの修正 - ([7f51445](https://github.com/taktiks2/bevy-life-game/commit/7f514455bddd871f37ead84fc3c548337e04516c)) - taktiks2
- cocogitto設定ファイルの追加 - ([3244886](https://github.com/taktiks2/bevy-life-game/commit/3244886208cc900f7feed6bf26ef8d20d0661629)) - taktiks2
- justfileの追加 - ([13bbb5a](https://github.com/taktiks2/bevy-life-game/commit/13bbb5a79cc36b1133a576b3d8835bc4d6144689)) - taktiks2
- ciのアクション追加 - ([561e5a3](https://github.com/taktiks2/bevy-life-game/commit/561e5a37cbcdd534859a85efe6502b7c3735ceac)) - taktiks2
- copilotのレビュールール追加 - ([e84c399](https://github.com/taktiks2/bevy-life-game/commit/e84c399ecf9c23f514322f3e37f9d09709a0dc98)) - taktiks2
- dependabotの設定追加 - ([464f4af](https://github.com/taktiks2/bevy-life-game/commit/464f4afcedef8031721c16f5fa67bac0fd071d23)) - taktiks2
- コメントの追記 - ([20ab48b](https://github.com/taktiks2/bevy-life-game/commit/20ab48b43fab0132da78e43394afdde7c362a87a)) - taktiks2
- CLAUDE.mdの生成 - ([c544477](https://github.com/taktiks2/bevy-life-game/commit/c544477da80cb6b98b1d43407415158c293157b8)) - taktiks2
- serenaの有効化 - ([cbcb467](https://github.com/taktiks2/bevy-life-game/commit/cbcb467b61ab40e2d40c5591ea5e309bc81c8836)) - taktiks2
#### Styling
- フォーマット - ([c5f72a7](https://github.com/taktiks2/bevy-life-game/commit/c5f72a703be613aa75a0aacb71c05a429c74e434)) - taktiks2

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).