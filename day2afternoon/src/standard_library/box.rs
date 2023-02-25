// Boxはヒープにおけるデータに対する所有ポインタである
// Box is like std::unique_ptr in C++.

fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
}
