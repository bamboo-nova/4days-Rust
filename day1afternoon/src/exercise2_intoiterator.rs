pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}

fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.into_iter();

    let v0: Option<String> = iter.next();
    println!("v0: {v0:?}");

    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    for word in &v {
        println!("word: {word}");
    }
    for word in v {
        println!("word: {word}");
    }
}
