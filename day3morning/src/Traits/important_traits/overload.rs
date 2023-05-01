// overloadingはstd::opsにあるトレイトを通して実装されている

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl std::ops::Add for Point {
    // 関連するOutputの型をtrait側で定義する
    // これがないと、missing `Output` in implementationとなる
    // type Output = Type;をつけろと言われる
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2);
}

// &Pointに対してAddを実装することもできる。この場合は、Add:addはselfを必要とする。
// もしもoverloadingするtype Tがcopyでない場合、同様に&Tがoverloadingされることを考慮すべき
// これは、不必要なcloningを避けている。

// Q. なぜOutputを関連したtypeにしているのか
// callerでtypeのパラメータは制御されているが、関連したtypeはトレイトのimplementerによって制御される

