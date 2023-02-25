// あらゆるsliceメソッドを使用できる
// VecはStringやhashMapと並ぶcollection型で、データはヒープで保存される
// よって、データ量はコンパイル時に知る必要はなく、実行してる過程で動的に変化する
// Vec<T>も汎用型で、Tを明示的に指定する必要がない。
// vec![..]はVec::new()の代わりに使用する標準マクロで、vectorに初期要素を追加する
// pop関数は最後を削除、getを使うとOptionが返ってくる。
// 全ての要素を反復的に変化させるときは、下記のように書く
// for e in &mut v { *e += 50; }
fn main() {
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());
}