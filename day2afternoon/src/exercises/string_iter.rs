// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use tracing::info;
use tracing_subscriber;
use itertools::{
    Itertools,
    EitherOrBoth::*,
};

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    // unimplemented!()
    info!(prefix, "input prefix");
    info!(request_path, "input request path");

    let split_prefix: Vec<&str> = prefix.split('/').collect();
    let split_request: Vec<&str> = request_path.split('/').collect();

    let mut check_flag: Vec<bool> = Vec::new();

    for pair in split_prefix.iter().zip_longest(split_request.iter()) {
        // match pair {
        //     Both(l, r) => println!("({:?}, {:?})", l, r),
        //     Left(l) => println!("({:?}, 0)", l),
        //     Right(r) => println!("(0, {:?})", r),
        // }
        match pair {
            Both(l, r) => if l == r {
                check_flag.push(true)
            } else if *l == "*" {
                check_flag.push(true)
            } else {
                check_flag.push(false)
            },
            Left(l) => check_flag.push(false),
            Right(r) => check_flag.push(true),
        }
    }
    if check_flag.iter().all(|&e| e) {
        info!("prefix matching is success!");
        true
    } else {
        info!("prefix matching is failed!");
        false
    }
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

fn main() {
    tracing_subscriber::fmt::init();

    prefix_matches("/v1/publishers", "/v1/publishers");
    prefix_matches("/v1/publishers", "/v1/publishers/abc-123");
    prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    );
}
