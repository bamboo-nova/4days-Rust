// Structs, enums, methods.
//Pattern matching: destructuring enums, structs, and arrays.
//Control flow constructs: if, if let, while, while let, break, and continue.
//The Standard Library: String, Option and Result, Vec, HashMap, Rc and Arc.
//Modules: visibility, paths, and filesystem hierarchy.

// Structs
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Tuple Structs
struct Point(i32, i32);
// これは下記のようなsingle-field wrappersとしても使われることがある
//struct PoundOfForce(f64);
//struct Newtons(f64);

// Field Shorthand Syntax
impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

fn main() {
    // Structs
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{} is {} years old", peter.name, peter.age);
    println!("{peter:?}");

    // Tuple Structs
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}
