// Rc内でデータを変異させる必要がある場合は、CellやRefCellなどの型にデータをラップする必要がある
// マルチスレッドコンテキストであればArcを参照
// 共有ポインタをWeakポインタにダウングレードして、ドロップされるサイクルを作る
// Weakポインタは共有ポインタの持つメモリへの参照を保持するが、所有権を持たないので知らない間に参照先メモリが解放される可能性がある

use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i64,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    let root = Rc::new(RefCell::new(Node {
        value: 42,
        parent: None,
        children: vec![],
    }));
    let child = Rc::new(RefCell::new(Node {
        value: 43,
        children: vec![],
        parent: Some(Rc::downgrade(&root))
    }));
    root.borrow_mut().children.push(child);

    println!("graph: {root:#?}");
}
