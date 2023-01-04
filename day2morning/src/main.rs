use rand::distributions::{Distribution, Uniform};
// Structs, enums, methods.
//Pattern matching: destructuring enums, structs, and arrays.
//Control flow constructs: if, if let, while, while let, break, and continue.
//The Standard Library: String, Option and Result, Vec, HashMap, Rc and Arc.
//Modules: visibility, paths, and filesystem hierarchy.

// Structs
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Tuple Structs
struct Point(i32, i32);
// これは下記のようなsingle-field wrappersとしても使われることがある
//struct PoundOfForce(f64);
//struct Newtons(f64);

// Field Shorthand Syntax
impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

// Enums
fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    die.sample(&mut rng)  // Chosen by fair dice roll. Guaranteed to be random.
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        return CoinFlip::Heads;
    } else {
        return CoinFlip::Tails;
    }
}

// Variant Payloads
// enumsは特定のイベントがやってきた時に対応するようなmatchと組み合わせてみると抽象化できるかも
enum WebEvent {
    PayLoad,                  // Variant without payload
    KeyPress(char),            // Tuple struct variant
    Click { x: i64, y: i64 },  // Full struct variant
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PayLoad        => println!("page loaded"),
        WebEvent::KeyPress(c)       => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

// Enum Sizes
use std::mem::{align_of, size_of};

macro_rules! dbg_size {
    ($t:ty) => {
        println!("{}: size {} bytes, align: {} bytes",
                stringify!($t), size_of::<$t>(), align_of::<$t>());
    };
}

enum Foo {
    A,
    B,
}

// Methods
// &selfは、メソッドがオブジェクトを永続的に借用することを意味する
// レシーバにはいくつかある
// &self: 普遍の参照を使用して借りる。再び使用できる
// mut &self: ユニークでミュータブルな参照を使用して借りる。再び使用できる。
// self: オブジェクトの所有権を取得して、メソッドが戻ったときに割り当てが解除されて解放される
// mut &selfは特定の一度しか使わない値など、例えばリアルタイムの処理をする時に活用できる
// また、新しい構造体を作成するときはnewメソッドを慣例的に用意する(pythonでいうなら__init__関数)
// 上の場合はレシーバは用意しない
impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name)
    }
}


fn main() {
    // Structs
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{} is {} years old", peter.name, peter.age);

    // Tuple Structs
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);

    // Field Shorthand Syntax
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");

    // Enums
    println!("You got: {:?}", flip_coin());

    // Variant Payloads
    let load = WebEvent::PayLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(press);
    inspect(click);

    // Enum Sizes
    dbg_size!(Foo);

    // Methods
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    peter.say_hello();
}
