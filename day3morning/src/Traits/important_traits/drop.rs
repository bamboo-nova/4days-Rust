// Dropを実装した値は、スコープ外に出たときに実行するコードを指定できる
// Rustでは、std::ops::Dropというトレイトを用いることでデストラクタを定義できる
// これで、メモリを解放したり、ファイルを閉じたり、ロックを解放することが可能

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropoing {}", self.name);
    }
}

fn main() {
    let a = Droppable { name: "a" };
    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(a);
    //a.drop(); // エラーになる
    println!("Exiting main");
}
// Drop::dropでselfを使ってない理由:
// std::mem::drop自体がブロックの終わりに呼び出されるため、スタックオーバーフローする
// https://users.rust-lang.org/t/stack-overflow-occurs-when-calling-drop/53357
