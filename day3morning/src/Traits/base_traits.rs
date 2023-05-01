// 型を抽象化するためにtraitを用いる。Goのインターフェースと仕組みは類似している
// Traitは予め実装されているメソッドとユーザーが自分で実装することが要求されるメソッドが指定できる
// デフォルトの実装をもつメソッドは実際のメソッドに依存できる

trait Greet {
    fn say_hello(&self);
}

struct Dog {
    name: String,
}

struct Cat;  // No name, cats won't respond to it anyway.

impl Greet for Dog {
    fn say_hello(&self) {
        println!("Wuf, my name is {}!", self.name);
    }
}

impl Greet for Cat {
    fn say_hello(&self) {
        println!("Miau!");
    }
}

fn main() {
    // 下記のように実装することはできるが、traitsを使って抽象化して保証しない場合、
    // Vec<Greet>を持つことができない
    let pets: Vec<Box<dyn Greet>> = vec![
        Box::new(Dog { name: String::from("Fido") }),
        Box::new(Cat),
    ];
    for pet in pets {
        pet.say_hello();
    }
    println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
    println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
    // dyn Greetは実装した動的な大きさの型についてコンパイラに伝えるための方法(8+8=16)
    // petsはGreetを実装したオブジェクトへのFat pointerを持っている
    // Fat Pointerは実際のオブジェクトへのポインタとその特定のオブジェクトのGreet実装のための
    // 仮想メソッド・テーブルへのポインタの二つで構成されている
    println!("{}", std::mem::size_of::<&dyn Greet>());
    println!("{}", std::mem::size_of::<Box<dyn Greet>>());
}
