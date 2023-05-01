struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    // 本来のiteratorsを書き換えて独自のiterator(next)を作れる
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn main() {
    let fib = Fibonacci { curr: 0, next: 1};
    for (i, n) in fib.enumerate().take(5) {
        println!("fib({i}): {n}");
    }
}