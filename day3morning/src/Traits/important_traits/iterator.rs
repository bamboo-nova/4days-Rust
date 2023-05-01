fn main() {
    //break文が使える。
    let _x = 7;
   
    //continue文が使える。
    for y in 0..10 {
        if y % 2 == 0 { continue; }
        
        println!("{}", y);
    }
}