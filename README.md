# librobo

## Variants
- [librobo](librobo) for std environment
- [librobo-nostd](librobo-nostd) for no\_std environment

## Description
`librobo`は環境非依存のロボット開発補助ライブラリです。

## Dependencies
ビルドするには以下のものが必要となります。

- Rust 1.31、1.30-beta、nightly-2018-09-13、もしくはそれ以上に新しいツールチェイン
- `cargo generate`サブコマンド([インストール方法(英語)](https://github.com/ashleygwilliams/cargo-generate#installation)
  参照)
- `rust-std`コンポーネント(コンパイル済み`core`クレート)  
  下記のコマンドでインストール可能。

``` console
$ rustup target add <TARGET_TRIPLE>
```

## License
このライブラリは[MIT License](LICENSE.md)の下で頒布されます。

## Contribution
コントリビューション大歓迎です。[メカトロニクス研究部会](https://github.com/mecha-natori)
の部員はもちろん、外部の方々も[Issue](https://github.com/mecha-natori/librobo/issues)や[Pull Request](https://github.com/mecha-natori/librobo/pulls)など送っていただいて構いません。

## Authors/Contributors
- Sora Tonami([@ms0503](https://github.com/ms0503)) - 開発メンバー
