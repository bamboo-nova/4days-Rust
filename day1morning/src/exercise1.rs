// Implicit Conversions
// .into()で暗に型を変換してくれる
// それぞれtrait From<T>で対応してるtypeが存在しており、float -> intの優先順位で成り立っている。
// つまり、f32(from)からi16(into)へ変換することは.into()からできないので注意
// main関数でx: f32 = 15とすると、.into()してもエラーになる。同じintやfloatならfromとintoの方向は関係ない。
fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
}
