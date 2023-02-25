#![allow(unused_variables, dead_code)]

// 非推奨(簡単な文字列でも0.1秒くらい遅くなってしまう)
// for i in 0..sentence.len() as usize {
//   println!("{}", sentence.chars().nth(i).unwrap());
// }

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
    cc_number.to_string().retain(|c| !c.is_whitespace());
    let remove_whitespace_cc_number: &str = &cc_number;
    // 数字以外が入ってきてるかどうかのチェック
    let check_numbers_only = match remove_whitespace_cc_number.parse::<u8>() {
        Ok(_) => true,
        Err(_) => false,
    };
    if !check_numbers_only {
        return false;
    }
    let mut index: usize = 0;
    let mut output_vecter: Vec<usize> = Vec::new();
    if remove_whitespace_cc_number.len() <= 2 {
        return false;
    }
    for c in remove_whitespace_cc_number.chars() {
        // .parse().unwrap()で文字列を数値に変換できる
        // cはchar型で一文字を表現してるので、parse()するためにStringに変換する
        // 数値からStringにしたいときは、&num.to_string();
        // さらに&strにしたい場合は&をつける
        // u8::try_from(255u32)
        let single_number: usize = c.to_string().parse().unwrap();
        let double_number: usize = single_number * 2;
        if index % 2 == 1 {
            let log_num: f32 = (double_number + 1) as f32;
            if log_num.log10() == 2.0 {
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
    if check_digit / 10 == 0 {
        true
    } else {
        false
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"))
}
