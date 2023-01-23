// <<<Pattern Matching>>>

// Destructuring Enums
// OkとErrは慣例
enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {} into two equal parts", n))
    }
}

// Destructuring Structs
struct Foo {
    x: (u32, u32),
    y: u32,
}

#[rustfmt::skip]
fn main() {
    // Pattern Matching
    let input = 'x';

    // '_'はワイルドカード
    match input {
        'q'                   => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9'             => println!("Number input"),
        _                     => println!("Something else"),
    }

    // Destructuring Enums
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }

    // Destructuring Structs
    let foo1 = Foo { x: (1, 2), y: 2 };
    match foo1 {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, i = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
    let foo2 = Foo { x: (2, 3), y: 2 };
    match foo2 {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, i = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
    let foo3 = Foo { x: (2, 3), y: 3 };
    match foo3 {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, i = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }

    // Destructuring Arrays
    let triple1 = [0, -2, 3];
    let triple2 = [1, -2, 3];
    let triple3 = [2, -2, 3];
    println!("Tell me about {triple1:?}");
    match triple1 {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..] => println!("First is 1 and the rest were ignored"),
        _ => println!("All elements were ignored"),
    }
    match triple2 {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..] => println!("First is 1 and the rest were ignored"),
        _ => println!("All elements were ignored"),
    }
    match triple3 {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..] => println!("First is 1 and the rest were ignored"),
        _ => println!("All elements were ignored"),
    }

    // Match Guards
    let pair = (2, -2);
    println!("Tell me about {pair:?}");
    match pair {
        (x, y) if x == y     => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _                    => println!("No correlation..."),
    }
}