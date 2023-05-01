// Closuresあるいはlambda表現は、typesを持ってないが、Fn, FnMut, FnOnceトレイトが使える
// FnOnceは一度だけ呼び出せる
// FnMutはキャプチャした値を変異させる可能性がある。複数回呼び出せるが、同時に呼び出すことはできない。
// Fn はキャプチャした値を消費も変異もしないが、同時に複数回呼び出すことが可能

fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

fn main() {
    let add_3 = |x| x + 3;
    let mul_5 = |x| x * 5;

    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("mul_5: {}", apply_with_log(mul_5, 20));
}