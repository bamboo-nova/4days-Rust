// Traits -> interfaceと同義で、typeにおけるmethodとして説明できる
pub trait Iterator {
    // Iteratorで高度な関連型を定義している。型エイリアスで型同義語を生成する。
    // これは、iter()メソッドのそれぞれの要素を定義している。実質<T>とも言える
    // type Item = i32;とするとassociated type defaultに引っかかる <https:/github.com/rust-lang/rust/issues/29661>
    type Item;
    // Optionにしているのでなんでも型を受け付ける
    // next自体は次の要素をSomeで返すか、なければNoneを返す
    // Iteratorについて知りたい場合は下記を参照
    // https://qiita.com/lo48576/items/34887794c146042aebf1
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    // 
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();
    println!("v[0]: {:?}", iter.next());
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());
    println!("No more items: {:?}", iter.next());

    // また、next自体がメソッドなので借用して&i8や&u8にする必要がある
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();
    let v0: Option<&i8> = iter.next();
    println!("v0: {v0:?}");

    let v: Vec<u8> = vec![10, 20, 30];
    let mut iter = v.iter();
    let v0: Option<&u8> = iter.next();
    println!("v0: {v0:?}");
}