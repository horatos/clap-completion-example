# clapクレートでシェル補完スクリプトを生成する

この記事では、2022年1月にリリースされたclapバージョン3とclap_completeクレートを使って、シェルの補完スクリプトを生成する方法を紹介します。

[clap](https://crates.io/crates/clap)はRustのコマンドライン引数パーサーです。Builderパターンによるパーサーの構築が素の使い方ですが、バージョン3でderiveマクロによるパーサーの構築が安定化されました。

Builderパターンによるパーサーの構築（この機能はBuilder APIと呼ばれています）では、以下のようなコードでパーサーを記述します。[構造体`App`](https://docs.rs/clap/latest/clap/struct.App.html)がビルダーであり、そのメソッドを呼び出すことで引数を追加していきます。

```rust
use clap::App;

let app = App::new("example")
    .arg(
        Arg::new("verbose")
            .short('v')
            .help("enable verbose mode")
    )
    .arg(
        Arg::new("file")
            .index(1)
    );
```

一方、deriveマクロによるパーサーの構築（この機能はDerive APIと呼ばれています）では、構造体に属性を付けることで引数を設定します。コマンドライン引数のパースに成功すると、deriveマクロを使った構造体にパースされた結果が格納されます。

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    file: PathBuf,
    /// enable verbose mode
    #[clap(short, long)]
    verbose: bool,
}
```

clapには[clap_complete](https://crates.io/crates/clap_complete)という関連するクレートがあります。clap_completeを使うことで、構築したパーサーからシェル補完スクリプトを自動で生成することができます。

clapとclap_completeを使ってシェル補完スクリプトを生成する方法について、次の流れで説明します。

1. 補完スクリプトの対象となる簡単なCLIプログラムを作成する。
2. clap_completeの使い方を解説し、補完スクリプトを生成するサブコマンドを実装する。
3. 生成した補完スクリプトをzshで使用する。

## メッセージを表示するプログラムを作成する

サンプルとして使うためのプログラムとして、メッセージを表示するだけのプログラムを作成します。この部分はこの記事の本題ではないので仕様とコードを掲載するだけにとどめます。

### メッセージを表示するプログラムの仕様

`greet`サブコマンドでメッセージを表示することにします。後で補完スクリプトを生成するための`completion`サブコマンドを追加するため、サブコマンドを分けることにします。

`greet`サブコマンドに引数が渡されたなかったときは、`Hello`を表示します。

```console
$ cargo run -- greet
Hello
```

`greet`サブコマンドに`-l LANGUAGE`または`--language LANGUAGE`オプションを渡すと、メッセージを表示する言語を変更します。このオプションの値`LANGUAGE`は`en`か`ja`のいずれかです。

```console
$ cargo run -- greet -l en
Hello
$ cargo run -- greet -l ja
こんにちは
```

`greet`サブコマンドに`-f FILE`または`--file FILE`オプションを渡すと、値`FILE`の内容を表示します。このオプションは`-l`オプションと併用できません。

```console
$ cargo run -- greet -f hey.txt
（hey.txtの内容が表示される。ただし、末尾の空白文字は取り除かれる。）
```

### メッセージを表示するプログラムの実装

実装は以下の通りになります。プロジェクト全体は[このリポジトリ](https://github.com/horatos/clap-completion-example/tree/step1)にあります。

```rust
use std::path::PathBuf;

use clap::{ArgEnum, Parser, Subcommand};

/// Greet command (example for clap_complete command).
#[derive(Parser,Debug)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand,Debug)]
enum Action {
    /// Greet some message.
    Greet {
        /// Language in which messages are shown.
        #[clap(long,short,arg_enum)]
        language: Option<Language>,
        /// File whose content is printed.
        ///
        /// The trailing whitespaces of the content are trimmed.
        #[clap(long,short,conflicts_with("language"))]
        file: Option<PathBuf>,
    },
}

#[derive(ArgEnum,Clone,Debug)]
enum Language {
    En,
    Ja,
}

impl Action {
    fn handle(self) {
        use Action::Greet;

        match self {
            Greet { language: None, file: None } => {
                println!("Hello");
            },
            Greet { language: Some(Language::En), .. } => {
                println!("Hello");
            },
            Greet { language: Some(Language::Ja), .. } => {
                println!("こんにちは");
            },
            Greet { file: Some(file), .. } => {
                let s = std::fs::read_to_string(&file).unwrap();
                println!("{}", s.trim_end());
            },
        }
    }
}

fn main() {
    Cli::parse().action.handle();
}
```

## clap_completeクレートを使って補完スクリプトを生成する

前節で作ったプログラムを拡張して補完スクリプトを生成するサブコマンドを追加します。サブコマンドの仕様を考える前にclap_completeが提供する関数やトレイトなどと見ておきましょう。

補完スクリプトの生成は[関数`generate`](https://docs.rs/clap_complete/latest/clap_complete/generator/fn.generate.html)が行います。関数`generate`は以下の四つの引数を受け取ります。

- `gen`
- `app`
- `bin_name`
- `buf`

まず、引数`gen`は[トレイト`Generator`](https://docs.rs/clap_complete/latest/clap_complete/generator/trait.Generator.html)を実装したオブジェクトでなければなりません。clap_completeはこのトレイトを実装する[列挙体`Shell`](https://docs.rs/clap_complete/latest/clap_complete/shells/enum.Shell.html)を提供しています。`Shell`は補完スクリプトの生成に対応しているシェルを表現する列挙体です。

次に、引数`app`は[clapの構造体`App`](https://docs.rs/clap/latest/clap/struct.App.html)のオブジェクトです。Builder APIでパーサーを構築する場合は素直に手に入りますが、Derive APIを使う場合は[トレイト`IntoApp`](https://docs.rs/clap/latest/clap/trait.IntoApp.html)を使うことで構造体`App`のオブジェクトを手に入れられます。

引数`bin_name`はコマンドの名前です。`Into<String>`を実装しているオブジェクトを渡す必要があります。この引数で渡す名前とバイナリの名前を一致させないと、シェルが補完スクリプトを呼び出すことができません。

引数`buf`は`Write`を実装しているオブジェクトを渡す必要があります。スクリプトの書き込み先をこの引数で渡します。
