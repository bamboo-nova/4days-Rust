use rand::distributions::{Distribution, Uniform};


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

fn main() {
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
}