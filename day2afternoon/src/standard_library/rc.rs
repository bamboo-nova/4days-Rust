// Rcは参照カウントによる共有ポインタで、複数の場所で同じデータを参照したい場合に使用する
// コピー可能だがオーバーヘッドが存在。所有権は共有するのでコピーが可能
// weakポインタはアクセスはできないのでムーブとコピーのみで所有権はないので注意
use std::rc::Rc;

fn main() {
    let mut a = Rc::new(10);
    // Cloneにすると、cargo testの時に下記のエラーが発生する
    // error[E0599]: no method named `Clone` found for struct `Rc<{integer}>` in the current scope
    // --> src/standard_library/rc.rs:8:19
    // |
    // 8 |     let mut b = a.Clone();
    // |                   ^^^^^ method not found in `Rc<{integer}>`
    // let mut b = a.Clone();
    // Rc::clone関数を呼び出し、引数としてaのRc<List>への参照を渡しています。
    // https://doc.rust-jp.rs/book-ja/ch15-04-rc.html
    let mut b = Rc::clone(&a);

    println!("a: {a}");
    println!("b: {b}");
}
