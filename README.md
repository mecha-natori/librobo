# `librobo`

`librobo`はSTM32F407xGTx向けのHALを各機構・部品向けに更に抽象化するライブラリです。

## Dependencies

ビルドするには以下のものが必要となります。

- Rust 1.31、1.30-beta、nightly-2018-09-13、もしくはそれ以上に新しいツールチェイン
- `cargo generate`サブコマンド([インストール方法(英語)](https://github.com/ashleygwilliams/cargo-generate#installation)
  参照)
- ARM Cortex-M4F向けターゲットの`rust-std`コンポーネント(コンパイル済み`core`クレート)  
  下記のコマンドでインストール可能。

``` console
$ rustup target add thumbv7em-none-eabihf
```

## License

このライブラリは[MIT License](LICENSE.md)の下で頒布されます。

## Contribution

コントリビューション大歓迎です。[メカトロニクス研究部会](https://github.com/mecha-natori)
の部員はもちろん、外部の方々もIssueやPull Requestなど送っていただいて構いません。

## Authors/Contributors

- Sora Tonami(@ms0503) - 開発メンバー
