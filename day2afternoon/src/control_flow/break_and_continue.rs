fn main() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    // 外側のループも含めてbreakしたい場合は'outerラベルを付ける
    // 'outerラベルをつけないと、下の例だと一番内側のループしか抜け出せない
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}