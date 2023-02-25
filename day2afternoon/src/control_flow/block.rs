// Blocks
// Block毎にvalueとtypeを持ち、valueはブロック内の最後の表現に従う(;区切り)
// 関数でも同じく作用する

fn main() {
    let x = {
        let y = 10;
        println!("y: {y}");
        let z = {
            let w = {
                3 + 4
            };
            println!("w: {w}");
            y * w
        };
        println!("z: {z}");
        z - y
    };
    println!("x: {x}");
}
