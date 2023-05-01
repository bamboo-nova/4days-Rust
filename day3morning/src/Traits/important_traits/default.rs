// Default traitは、traitのデフォルトの実装を提供している
// 直接実装もできるし、#[derive(Default)]を使って派生させても良い
// 全てのフィールドがデフォルトの値に設定したインスタンスが生成されるが、構造体の全ての型もDefaultを実装する必要がある
// 部分的な構造体のコピーもうまく機能するし、unwrap_or_defaultなどの便利なメソッドが用意されている

// warning: fields `x`, `y`, and `z` are never read
// 上の警告が出てくるのでallow(dead_code)を追加する
#[derive(Debug, Default)]
#[allow(dead_code)]
struct Derived {
    x: u32,
    y: String,
    z: Implemented,
}

#[derive(Debug)]
struct Implemented(String);

impl Default for Implemented {
    fn default() -> Self {
        // zにデフォルトとして追加される
        Self("John Smith".into())
    }
}

fn main() {
    let default_struct: Derived = Default::default();
    println!("{default_struct:#?}");

    let almost_default_struct = Derived {
        y: "Y is set!".into(),
        ..Default::default()
    };
    println!("{almost_default_struct:#?}");

    let nothing: Option<Derived> = None;
    println!("{:#?}", nothing.unwrap_or_default());
}
