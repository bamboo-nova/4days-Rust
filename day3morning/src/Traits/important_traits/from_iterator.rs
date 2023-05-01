// fn collect<B>(self) -> B where B: FromIterator<Self::Item>, Self: Sized
// Iterator<Item = Result<V, E>> から Result<Vec<V>, E>へ変換している。

fn main() {
    let primes = vec![2, 3, 5, 7];
    let _prime_squares = primes
        .into_iter()
        .map(|prime| prime * prime)
        .collect::<Vec<_>>();
    println!("{:?}", _prime_squares)
}