// https://google.github.io/comprehensive-rust/std/box-recursive.html

// Boxを使用せずListを直接埋め込もうとすると、コンパイラはメモリ上の構造体の固定サイズを計算せず、無限大に見えてしまう。
// Boxは通常のポインタと同じサイズで、ヒープ上のListの次の要素を指すだけなので、この問題を解決することができる。
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}
