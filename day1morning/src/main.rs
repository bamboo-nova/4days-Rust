// Methods(last topic of 1st day's morning)
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height // Return value
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

// Generic Function
// 関数は単一の実装で、常にパラメータの数は固定されていて、型も固定されている。
// また、関数のデフォルト値は許容されていない(マクロが代替になっている)
// パラメータの型については、ジェネリックである程度対応できる。
fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 {
        a
    } else {
        b
    }
}

fn main() {
    // Basic calculation
    let mut x: i32 = 6;
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        println!("-> {x}");
    }
    println!();

    // Array Assignment
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a);

    // Tuple Assignment
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);

    // Reference like C++
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x; // &mut≒可変長としてxの参照値を受け取る
    *ref_x = 20; // 参照先の値に20を代入する。
    println!("x: {x}");

    // Dangling References(解放)
    // 詳細: https://zenn.dev/ucwork/articles/6de5c9c2257f2d
    // この一行は「maybe it is overwritten before being read?」という警告が出てくる
    // 一度も使わない変数どころか、一度も使わない値(ここでは20)に対してもwarningを出してくるので愛が重い
    let mut ref_x: i32 = 20;
    {
        let x: i32 = 10;
        ref_x = x;
        // lifetimeの短い参照したxを入れるとエラーになる。
        // ref_x = &xはできない(参照は値を借りるだけであり、&xそのものを渡してxを解放はできないので注意)
    }
    println!("ref_x: {ref_x}");

    // Slices
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");

    let s: &i32 = &a[3]; // 単一要素の場合は[]はいらないので注意
    println!("s: {s:?}");

    // String vs &str
    // &strは文字列スライスにおけるimmutableな参照
    // Stringはmutableな文字列バッファ
    let s1: &str = "Hello";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    // FizzBuzz(Function)
    fizzbuzz_to(20);

    // Rectangle(Method)
    let mut rect = Rectangle {
        width: 10,
        height: 5,
    };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());

    // Function Overloading is not supported
    // But, function parameters can be generic.
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0 // The last expression is the return value
}

fn fizzbuzz(n: u32) -> () {
    // No return value means returning
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true, true) => println!("fizzbuzz"),
        (true, false) => println!("fizz"),
        (false, true) => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}

fn fizzbuzz_to(n: u32) {
    // '-> ()' is normally omitted
    for n in 1..=n {
        fizzbuzz(n);
    }
}
