// Type Inference
fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

// Const
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    
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

    //
}
