// Methods
// &selfは、メソッドがオブジェクトを永続的に借用することを意味する
// レシーバにはいくつかある
// &self: 普遍の参照を使用して借りる。再び使用できる
// mut &self: ユニークでミュータブルな参照を使用して借りる。再び使用できる。
// self: オブジェクトの所有権を取得して、メソッドが戻ったときに割り当てが解除されて解放される
// mut &selfは特定の一度しか使わない値など、例えばリアルタイムの処理をする時に活用できる
// また、新しい構造体を作成するときはnewメソッドを慣例的に用意する(pythonでいうなら__init__関数)
// 上の場合はレシーバは用意しない
impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name)
    }
}

fn main() {
    // Methods
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    peter.say_hello();
}
