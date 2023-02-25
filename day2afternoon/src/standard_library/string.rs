// StringではDeref<Target = str>が実装されてるので、strメソッドが全て利用できる
// lenはバイトのStringのサイズを返す(文字列の長さではないので注意)
// charsは実際の文字列に対するイテレータを返す
fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    // capacity()で確保する容量を指定できる
    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!("s3: len = {}, number of chars = {}", s3.len(),
        s3.chars().count());
}