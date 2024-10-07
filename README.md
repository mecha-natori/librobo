# librobo

![robo 0.2.0](https://img.shields.io/badge/robo-v0.2.0-orange)
![robo_compat 0.1.1](https://img.shields.io/badge/robo__compat-v0.1.1-orange)
![robo_macro 0.1.1](https://img.shields.io/badge/robo__macro-v0.1.1-orange)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/mecha-natori/librobo/rust.yml?branch=main)
[![Docuemntation - robo](https://img.shields.io/badge/docs-robo-blue)](https://mecha-natori.github.io/librobo/robo)
[![Docuemntation - robo_compat](https://img.shields.io/badge/docs-robo__compat-blue)](https://mecha-natori.github.io/librobo/robo_compat)
[![Docuemntation - robo_macro](https://img.shields.io/badge/docs-robo__macro-blue)](https://mecha-natori.github.io/librobo/robo_macro)

## Crates

- [robo](librobo) is a main crate
- [robo_compat](librobo-compat) is a compatibility library
- [robo_macro](librobo-macro) is a macro library

## Description

`librobo`は環境非依存のロボット開発補助ライブラリです。

## Dependencies

ビルドするには以下のものが必要となります。

- Rust 1.56以上
- `rust-std`コンポーネント(コンパイル済み`core`クレート)  
  下記のコマンドでインストール可能。
  ``` console
  $ rustup target add <TARGET_TRIPLE>
  ```

## License

このライブラリは[MIT License](LICENSE.md)の下で頒布されます。

## Contribution

コントリビューション大歓迎です。[メカトロニクス研究部会](https://github.com/mecha-natori)
の部員はもちろん、外部の方々も[Issue](https://github.com/mecha-natori/librobo/issues)
や[Pull Request](https://github.com/mecha-natori/librobo/pulls)など送っていただいて構いません。

## Authors/Contributors

- Sora Tonami([@ms0503](https://github.com/ms0503)) - 開発メンバー
