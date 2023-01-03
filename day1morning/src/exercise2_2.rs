#![allow(unused_variables, dead_code)]

fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut t = vec![Vec::with_capacity(matrix[1].len()); matrix[0].len()];
    for row in matrix {
        for i in 0..row.len() {
            t[i].push(row[i]);
        }
    }
    t
}

// &をつけないと借用してないのでmatrixの所有権は移動したことになり、
// transposeでmatrixが引数として使えなくなってしまう
fn pretty_print(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        println!("{:?}", row)
    }
}

fn main() {
    let matrix = vec![
        vec![101, 102, 103],
        vec![201, 202, 203],
        vec![301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}