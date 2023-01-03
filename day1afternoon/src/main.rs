// Type Inference
fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

// Const
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(0);

// Static
static BANNER: &str = "Welcome to RustOS 3.14";

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

// Ownership
struct Point(i32, i32);

// Moves in Function Calls
fn say_hello(name: String) {
    println!("Hello {name}");
}
// このケースならBorrowingなので、関数内のスコープにおける変数に所有権が移行しないので、何度も呼び出せる
// 通常なら関数が終わったらスコープ内の変数は解放されてしまうので使えなくなる
// 一応、Borrowingのセクションで説明がある
fn say_hello_fix(name: &String) {
    println!("Hello {name}");
}

// Copying and Cloning
// 基本：https://qiita.com/Papillon6814/items/97c175fd94f0107d3821
// Copy Traitを入れることで、copy semanticsのタイプを受け入れられる
#[derive(Copy, Clone, Debug)]
struct PointCopy(i32, i32);

// Borrowing
#[derive(Debug)]
struct PointBorrowing(i32, i32);

fn add(p1: &PointBorrowing, p2: &PointBorrowing) -> PointBorrowing {
    PointBorrowing(p1.0 + p2.0, p1.1 + p2.1)
}

// Lifetimes in Function Calls
// 'aはgeneric paramterでコンパイラによって参照される
// &'aは、参照されたPointBorrowingでライフタイムaの区間で最低限有効であることを示す
fn left_most<'a>(p1: &'a PointBorrowing, p2: &'a PointBorrowing) -> &'a PointBorrowing {
    if p1.0 < p2.0 { p1 } else { p2 }
}

// Lifetimes in Data Structures
// 構造体に参照(&)があった場合もlifetime annotation('a)がないとエラーになるのて注意
#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}!");
}

fn main() {
    // Variable
    // mutにしないとx=20でエラーになる
    let mut x: i32 = 10;
    println!("x: {x}");
    x = 20;
    println!("x: {x}");

    // Type Inference
    let x = 10;
    let y = 20;
    takes_u32(x);
    takes_i8(y);

    // Const
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");

    // Static
    println!("{BANNER}");

    // Scopes and Shadowing
    // 外部スコープの変数と同じスコープの変数をシャドウすることができる。
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }
    println!("after: {a}");

    // Memory Managementについて
    // CやC++では手動でメモリ管理を実施して、pythonやGoでは自動的にメモリ管理される
    // Rustでは、オーナーシップ(Ownership)の概念とスコープによって以下のようにメモリ管理される
    // 「コンパイル時に正しいメモリ管理を実施することにより、完全な制御と安全性を実現する」

    // Stack(Ex: Python, Go) vs Heap(Ex: Rust)
    // Stack: ローカル変数においてメモリの連続した領域
        // コンパイラやOSが自動的に割り当てて解放する。
        // よって、プログラムのタスクやスレッド、関数内の変数やアドレスに対して割り当てられる
        // 値はコンパイル時に固定され、関数の呼び出しに従うので管理は簡単で、メモリに忠実
        // また、スタックポイントが移動するだけなので非常に早い
        // しかし、使用するサイズが決定しているので使いまくるとスタックオーバーフローするリスクがある
    // Heap: 関数呼び出しの外側で値を保存する
        // メモリ領域を動的に確保、解放を繰り返す
        // メモリが必要になった際に確保をするので、不要になったものを解放する処理は自ら行う必要がある
        // プログラム実行時にOSからソフトウェアに対して一定量のヒープ領域が確保される
        // 実行時に動的なサイズを持ち、スタックより遅く、メモリの保証がない
        // しかし、使用するサイズとタイミングはアプリケーションで決定できる
    // 所有権では、どの部分のコードがどのヒープ上のデータを使用してるのか分かる
    // 従って、ヒープ上の重複するデータを最小化でき、メモリ不足を防ぐためにヒープ上の未使用のデータを排除できる
    // メモリを所有している変数がスコープを抜けたら、メモリは自動的に返還される。
    // この仕組みは今まで無かった(ガベージコレクタがある言語では、使用されないメモリを検知して片付ける。
    // CやC++のような言語では、メモリがもう使用されないことを見計らって、明示的に返還するコードを呼び出す必要があった汗

    // Stack Memory
    let s1 = String::from("Hello");

    // CやC++では手動でメモリ管理をするので、mallocとfreeを行う必要があった
    // Javaやpythonではメモリの割り当てが必要なく、ガーベジコレクタは、未使用のメモリを見つけて自動的に解放している
    // Rustでは所有権をスコープで管理することによってうまく機能させている
    // Javaのように安全で正確だが、ガーベジコレクションがいらない。C++のようなスコープだが、コンパイラが完全準拠
    // C++のようなランタイムのオーバーヘッドがない
    // 詳細はこちら：https://google.github.io/comprehensive-rust/memory-management/comparison.html


    // Ownership
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    // println!("y: {}", p.1);  // エラーになる(scopeの問題)

    // Move Semantics
    // s1からs2へオーナーシップが遷移して、s1へはアクセスできなくなる
    // s2がスコープから外れると、文字列データは解放される
    // Detail: https://google.github.io/comprehensive-rust/ownership/moved-strings-rust.html
    // ちなみにC++ではコピーを複製させて解決している(https://google.github.io/comprehensive-rust/ownership/double-free-modern-cpp.html)
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");  // error

    // Moves in Function Calls
    let name = String::from("Alice");
    say_hello_fix(&name);
    say_hello_fix(&name);
    say_hello(name);
    // say_hello(name);  // Error

    // Copying and Cloning
    // 割り当て後にp1とp2は各々自身のデータを所有する
    // 明示的にコピーしたい場合はp1.clone()でも良い
    let p1 = PointCopy(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");

    // Borrowing
    let p1 = PointBorrowing(3, 4);
    let p2 = PointBorrowing(10, 20);
    // 二つのポインタを借りて新しいポイントを返す
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");

    // Shared and Unique Borrows
    // Rustのborrowingは制約があって、任意の時点で一つ以上の&T値を保つことができます
    // なので、関数の引数でBorrowするのは問題ないです
    // しかし、&mut Tの値を保つことができるのは一つだけなので注意(~c: &mut~の行)
    let mut a: i32 = 10;
    // let b: &i32 = &a;

    {
        // コメントを外すとmutable borrow occurs hereが出てくる
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    // println!("b: {b}");

    // Lifetimes
    // 借りた値にはライフタイムがある
    // Rustのver1.0以前はライフタイムを自明に書く必要があったが、今は自明と判断されれば省略できる
    // &'aのように書けば自明になる(&'a Point, &'document strなど)
    // ライフタイムはコンパイラによって参照されるので、こちら側から割り当てることはできない。
    // Lifetimes in Function Calls
    let p1 = PointBorrowing(10, 10);
    let p2 = PointBorrowing(20, 20);  // Put into different scope
    let p3: &PointBorrowing = left_most(&p1, &p2);
    println!("left-most point: {:?}", p3);

    // Lifetimes in Data Structures
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    // textの所有権が関数へ移動してしまうので、
    // eraseを実行するとtextを参照しているfoxが参照先がなくてエラーになる。
    erase(text);
    println!("{fox:?}");
    println!("{dog:?}");

}
