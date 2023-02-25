fn main() {
    // while expression
    let mut x = 10;
    while x != 1 {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
    }
    println!("Final x: {x}");

    // while let expressions(全ての項目に対して反復処理を実行させる)
    // v.iter()が返すイテレータは呼び出すたびにOption<i32>を返す
    // 終了するまでSome(x)を返し、終了後はNoneを返す
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
}