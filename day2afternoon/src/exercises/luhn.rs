#![allow(unused_variables, dead_code)]

use tracing::info;
use tracing_subscriber;

macro_rules! round {
    ($x:expr, $scale:expr) => (($x * $scale).round() / $scale)
}
macro_rules! ceil {
    ($x:expr, $scale:expr) => (($x * $scale).ceil() / $scale)
}
macro_rules! floor {
    ($x:expr, $scale:expr) => (($x * $scale).floor() / $scale)
}

pub fn divide_digit_and_sum(n: usize) -> usize {
    if n > 0 {
        let mut num = n;
        let mut result = Vec::new();
        while num != 0 {
            result.push((num % 10).try_into().unwrap());
            num /= 10;
        }
        result.reverse();
        result.iter().sum()
    } else {
        n
    }
}

pub fn luhn(cc_number: &str) -> bool {
    info!(cc_number, "input numbers for check digit");

    let remove_cc_number: String = cc_number.chars().filter(|c| !c.is_whitespace()).collect();
    let preprocessed_cc_number: &str = &remove_cc_number;
    // 数字以外が入ってきてるかどうかのチェック(u64にしないとパースできずErrになるので注意)
    let check_numbers_only = match preprocessed_cc_number.parse::<u64>() {
        Ok(_) => true,
        Err(_) => false,
    };
    if !check_numbers_only {
        info!("Input cc_number includes except for numbers.");
        return false;
    }
    let mut index: usize = 0;
    let mut output_vecter: Vec<usize> = Vec::new();
    if preprocessed_cc_number.len() < 2 {
        info!("The length of cc_number is less than 2.");
        return false;
    }
    // charsで一文字ずつ取得する
    // 参考: rev()なら良いけどreverseは破壊してしまう。iter経由のrev()なら破壊しないので大丈夫
    // https://zenn.dev/megeton/articles/fb6266bcb6aa1b
    // また、下記の書き方は非推奨(簡単な文字列でも0.1秒くらい遅くなってしまう)
    // for i in 0..sentence.len() as usize {
    //   println!("{}", sentence.chars().nth(i).unwrap());
    // }
    for c in preprocessed_cc_number.chars().rev() {
        // .parse().unwrap()で文字列を数値に変換できる
        // cはchar型で一文字を表現してるので、parse()するためにStringに変換する
        // 数値からStringにしたいときは、&num.to_string();
        // さらに&strにしたい場合は&をつける
        // メモ：u8::try_from(255u32)
        let single_number: usize = c.to_string().parse().unwrap();
        let double_number: usize = single_number * 2;
        if index % 2 == 1 {
            let log_num: f32 = (double_number + 1) as f32;
            if floor!(log_num.log10(), 10.0) >= 1.0 {
                output_vecter.push(divide_digit_and_sum(double_number));
            } else {
                output_vecter.push(double_number);
            }
        } else {
            let single_number: usize = c.to_string().parse().unwrap();
            output_vecter.push(single_number);
        }
        index += 1;
    }
    let check_digit: usize = output_vecter.iter().sum();
    if check_digit % 10 == 0 {
        info!("cc_number is valid!");
        true
    } else {
        info!("cc_number is not valid.");
        false
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"))
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("   "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
    // install global collector configured based on RUST_LOG env var.
    // tracingの初期化はmain関数で行うこと
    // https://zenn.dev/belle/articles/900e490ae8dbfe
    // https://blog.ymgyt.io/entry/how-tracing-and-tracing-subscriber-write-events/
    tracing_subscriber::fmt::init();

    luhn(" 0 0 ");
    luhn("4263 9826 4026 9299");
    luhn("4223 9826 4026 9299");
}
