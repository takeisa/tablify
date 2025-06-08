# Tablify

入力テキストをテーブル形式にフォーマットするRustで書かれたコマンドラインアプリケーションです。標準入力からテキストを読み取り、フォーマットされたテーブルを標準出力に出力します。

[English README is here](README.md)

## 機能

- 標準入力からテキストを読み取り
- テキストをテーブル構造にフォーマット
- フォーマットされたテーブルを標準出力に出力
- カスタム区切り文字と正規表現パターンに対応
- 全角文字（日本語、中国語、韓国語）を正しく処理
- ヘッダー行とカスタム列名をサポート

## インストール

```bash
git clone https://github.com/yourusername/tablify.git
cd tablify
cargo build --release
```

## 使用方法

### 基本的な使用方法

デフォルトでは、入力テキストはTAB文字を区切り文字として分割されます：

```bash
$ echo -e "apple\t100\norange\t200" | cargo run
| apple  | 100 |
| orange | 200 |
```

### コマンドラインオプション

#### 区切りオプション

**カスタム文字区切り (`-s, --separator`)**
```bash
$ echo -e "apple 100\norange 200" | cargo run -- -s ' '
| apple  | 100 |
| orange | 200 |
```

**正規表現パターン (`-p, --separator-pattern`)**
```bash
$ echo -e "apple   100\norange  200" | cargo run -- -p '\s+'
| apple  | 100 |
| orange | 200 |
```

#### ヘッダーオプション

**ヘッダー行 (`--header`)**
```bash
$ echo -e "item\tprice\napple\t100\norange\t200" | cargo run -- --header
| item   | price |
+--------+-------+
| apple  | 100   |
| orange | 200   |
```

**カスタム列名 (`--columns`)**
```bash
$ echo -e "apple\t100\norange\t200" | cargo run -- --columns "fruit,price"
| fruit  | price |
+--------+-------+
| apple  | 100   |
| orange | 200   |
```

### 全角文字サポート

Tablifyは、アジア言語でよく使用される全角文字を正しく処理します：

```bash
$ echo -e "りんご\t100\nオレンジ\t200" | cargo run
| りんご   | 100 |
| オレンジ | 200 |
```

### 実用的な使用例

**CSVファイルをテーブル表示**
```bash
$ cat data.csv | tablify -s ','
```

**スペース区切りのファイルをヘッダー付きで表示**
```bash
$ cat data.txt | tablify -p '\s+' --header
```

**カスタム列名でテーブル作成**
```bash
$ ps aux | head -5 | tablify --columns "USER,PID,CPU,MEM,VSZ,RSS,TTY,STAT,START,TIME,COMMAND"
```

## 依存関係

- [clap](https://crates.io/crates/clap) - コマンドライン引数解析
- [regex](https://crates.io/crates/regex) - 正規表現サポート
- [unicode-width](https://crates.io/crates/unicode-width) - Unicode文字の適切な幅計算

## テスト

テストスイートを実行：

```bash
cargo test
```

## ライセンス

このプロジェクトはオープンソースで、[MIT License](LICENSE)の下で利用可能です。

## コントリビューション

コントリビューションを歓迎します！お気軽にPull Requestを送信してください。

## 特別な考慮事項

- このアプリケーションは全角文字（日本語文字、ひらがな、カタカナなど）を正しく処理します。これらはASCII文字と表示幅が異なるため、適切な幅計算が重要です。
- unicode-widthライブラリを使用して、文字の表示幅を正確に計算しています。