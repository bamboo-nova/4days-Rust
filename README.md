# 4days-Rust
Rustの練習

Four day Rust Cource
https://google.github.io/comprehensive-rust/welcome.html

## Rustのインストール方法(Mac)
https://qiita.com/notakaos/items/9f3ee8a3f3a0caf39f7b

### 事前準備
Macの場合は予めXcode command line toolsが必要なので、ターミナルで下記を入力

```
xcode-select --install
```

### rustupのインストール
```
# rustupインストールおよびrust環境のセットアップ
brew install rustup-init
rustup-init
# シェルの再起動
exec $SHELL -l
# 確認
rustup --version
出力例：
rustup 1.25.1 (2022-07-12)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.64.0 (a55dd71d5 2022-09-19)`
```

### rustc(Rustコンパイラ)とビルドシステム兼パッケージ管理としてcargoの確認
rustupをインストールしたと同時にインストールされているはずなので確認
```
rustc --version
#=> rustc 1.64.0 (a55dd71d5 2022-09-19)

cargo --version
#=> cargo 1.64.0 (387270bc7 2022-09-16)
```

## コマンドの実行方法
### src以下にmain.rsしかない場合
```rust
cargo run
```

### src以下に複数の.rsファイルが存在する場合
まず、Cargo.tomlを修正する(day1morningフォルダのCargo.tomlファイル)
```
[[bin]]
name = "main"
path = "src/main.rs"

[[bin]]
name = "exercise1"
path = "src/exercise1.rs"

[[bin]]
name = "exercise2_1"
path = "src/exercise2_1.rs"

[[bin]]
name = "exercise2_2"
path = "src/exercise2_2.rs"
```

そして、下記のように--binでnameを指定して実行する
```
cargo run --bin exercise2_2
```

参考：https://zenn.dev/kazu8/articles/82a0140e054f8d

