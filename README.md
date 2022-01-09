# clapクレートでシェル補完スクリプトを生成する

この記事では、2022年1月にリリースされたclapバージョン3とclap_completeクレートを使って、シェルの補完スクリプトを生成する方法を紹介します。

[clap](https://crates.io/crates/clap)はRustのコマンドライン引数パーサーです。Builderパターンによるパーサーの構築が素の使い方ですが、バージョン3でderiveマクロによるパーサーの構築が安定化されました。

Builderパターンによるパーサーの構築（この機能はBuilder APIと呼ばれています）では、以下のようなコードでパーサーを記述します。構造体`App`がビルダーであり、そのメソッドを呼び出すことで引数を追加していきます。

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
