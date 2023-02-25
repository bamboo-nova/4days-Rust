// カンマがないが、ifとelseでブロック毎に分かれてるので問題ない

fn main() {
    // General if expression.
    let mut x = 10;
    x = if x % 2 == 0 {
        x / 2
    } else {
        3 * x + 1
    };
    println!("x = {x}");

    // if let expressions
    // letはmatchよりも簡潔で、一つのケースにしか興味がない場合に有効
    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }

    // if letをmatchで書いた場合
    // 全ての可能性を網羅したい場合はmatchが良い
    let arg = std::env::args().next();
    match arg {
        Some(value) => println!("Program name: {value}"),
        _ => println!("Missing name?"),
    }
}
